use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
// use prettytable::{format, Cell, Row, Table};
use regex::Regex;
use serde_json::Value;
use yaml_rust::yaml::{Array, Hash};
use yaml_rust::{Yaml, YamlEmitter};

use comfy_table::ContentArrangement;
use comfy_table::{Table, Row, Cell, Color};
use comfy_table::TableComponent::*;

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub enum TableHeader {
    NamedFields { fields: Vec<String> },
    SingleUnnamedColumn,
}

pub struct JsonTable {
    headers: TableHeader,
    values: Vec<Vec<Value>>,
}

impl JsonTable {
    pub fn new(headers: Option<TableHeader>, root: &Value) -> JsonTable {
        let rows: Vec<Value> = match root {
            Value::Array(arr) => arr.to_owned(), // TODO: is it possible to avoid cloning here?
            _ => vec![root.to_owned()],
        };

        let headers = headers.unwrap_or_else(|| infer_headers(&rows));
        let mut values = Vec::new();

        match &headers {
            TableHeader::NamedFields { fields } => {
                for row in rows {
                    values.push(
                        fields
                            .iter()
                            .map(|h| row.get(h).unwrap_or(&Value::Null).to_owned())
                            .collect(),
                    )
                }
            }
            TableHeader::SingleUnnamedColumn => {
                for row in rows {
                    values.push(vec![row.to_owned()])
                }
            }
        }
        JsonTable { headers, values }
    }
}

fn infer_headers(arr: &Vec<Value>) -> TableHeader {
    match arr.first() {
        Some(Value::Object(obj)) => TableHeader::NamedFields {
            fields: obj.keys().map(|h| h.to_owned()).collect(),
        },
        _ => TableHeader::SingleUnnamedColumn,
    }
}

#[derive(Debug)]
pub struct ColorizeSpec {
    field: String,
    value: String,
    style: String, /* this field refers to style-set method of old prettytable crate */
}

impl ColorizeSpec {
    pub fn parse(s: &String) -> GenericResult<ColorizeSpec> {
        let re = Regex::new(r"^([^:]+):(.+):([a-zA-Z]+)$")?;
        match re.captures(s) {
            Some(captures) => {
                let field = captures
                    .get(1)
                    .ok_or("wrong regular expression...")?
                    .as_str()
                    .to_string();
                let value = captures
                    .get(2)
                    .ok_or("wrong regular expression...")?
                    .as_str()
                    .to_string();
                let style = captures
                    .get(3)
                    .ok_or("wrong regular expression...")?
                    .as_str()
                    .to_string();
                Ok(ColorizeSpec {
                    field,
                    value,
                    style,
                })
            }
            _ => Err("wrong colorize expression. Should be in the form of : 'field:value:spec'")?,
        }
    }
}

pub trait Printer {
    fn print(&self, data: &JsonTable) -> GenericResult<()>;
}

fn json_to_yaml(value: &Value) -> Yaml {
    match value {
        Value::Object(obj) => {
            let mut hash = Hash::new();
            for (key, value) in obj {
                hash.insert(Yaml::String(key.to_owned()), json_to_yaml(value));
            }
            Yaml::Hash(hash)
        }
        Value::Array(arr) => {
            let arr = arr.iter().map(|e| json_to_yaml(e)).collect::<Vec<_>>();
            Yaml::Array(Array::from(arr))
        }
        Value::Null => Yaml::Null,
        Value::Bool(e) => Yaml::Boolean(e.to_owned()),
        Value::Number(n) => Yaml::Real(format!("{}", n)),
        Value::String(s) => Yaml::String(s.to_owned()),
    }
}

#[derive(Debug)]
pub enum PlainTextTableFormat {
    Default,
    Markdown,
}

#[derive(Debug)]
pub enum HtmlTableFormat {
    Raw,
    Styled,
}

#[derive(Debug)]
pub enum TableFormat {
    PlainText(PlainTextTableFormat),
    Html(HtmlTableFormat),
}

fn pprint_table_cell(value: &Value) -> GenericResult<String> {
    match value {
        Value::String(s) => Ok(s.to_string()),
        Value::Object(_) | Value::Array(_) => {
            let mut res = String::new();
            {
                let yaml_form = json_to_yaml(value);
                let mut emitter = YamlEmitter::new(&mut res);
                emitter.dump(&yaml_form)?;
            }
            Ok(res.trim_start_matches("---\n").to_string())
        }
        _ => Ok(serde_json::to_string(value)?),
    }
}

/// A wrapper of comfy-table
#[derive(Debug)]
pub struct PlainTextTable {
    inner_table: Table,
}

impl fmt::Display for PlainTextTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner_table.lines().collect::<Vec<_>>().join("\n"))
    }
}

impl PlainTextTable {
    pub fn new() -> PlainTextTable {
        PlainTextTable { inner_table: Table::new() }
    }

    pub fn set_content_arrangement(&mut self, arrangement: ContentArrangement) -> &mut Self {
        self.inner_table.set_content_arrangement(arrangement);

        self
    }

    pub fn set_header<T: Into<Row>>(&mut self, row: T) -> &mut Self {
        self.inner_table.set_header(row);

        self
    }

    pub fn add_row<T: Into<Row>>(&mut self, row: T) -> &mut Self {
        self.inner_table.add_row(row);

        self
    }

    pub fn set_corners(
        &mut self, 
        tlc: char, 
        trc: char, 
        blc: char, 
        brc: char,
    ) -> &mut Self {
        self.inner_table
        .set_style(TopLeftCorner, tlc)
        .set_style(TopRightCorner, trc)
        .set_style(BottomLeftCorner, blc)
        .set_style(BottomRightCorner, brc);

        self
    }

    pub fn set_intersections(
        &mut self,
        lhi: char,
        rhi: char,
        lbi: char,
        rbi: char,
        tbi: char,
        mhi: char,
        mi: char,
        bbi: char,
    ) -> &mut Self {
        self.inner_table
        .set_style(LeftHeaderIntersection, lhi)
        .set_style(RightHeaderIntersection, rhi)
        .set_style(LeftBorderIntersections, lbi)
        .set_style(RightBorderIntersections, rbi)
        .set_style(TopBorderIntersections, tbi)
        .set_style(MiddleHeaderIntersections, mhi)
        .set_style(MiddleIntersections, mi)
        .set_style(BottomBorderIntersections, bbi);

        self
    }

    pub fn set_lines(
        &mut self,
        hel: char,
        hol: char,
        vl: char,
    ) -> &mut Self {
        self.inner_table
        .set_style(HeaderLines, hel)
        .set_style(HorizontalLines, hol)
        .set_style(VerticalLines, vl);

        self
    }

    pub fn set_borders(
        &mut self,
        lb: char,
        rb: char,
        tb: char,
        bb: char,
    ) -> &mut Self {
        self.inner_table
        .set_style(LeftBorder, lb)
        .set_style(RightBorder, rb)
        .set_style(TopBorder, tb)
        .set_style(BottomBorder, bb);

        self
    }

    pub fn get_inner_table(&mut self) -> &mut Table {
        // get the wrapped table object to customize it with more flexibility
        &mut self.inner_table
    }
}

pub struct PlainTextTablePrinter {
    colorize: Vec<ColorizeSpec>,
    format: PlainTextTableFormat,
}

impl PlainTextTablePrinter {
    pub fn new(colorize: Vec<ColorizeSpec>, format: PlainTextTableFormat) -> PlainTextTablePrinter {
        PlainTextTablePrinter { colorize, format }
    }
}

impl Printer for PlainTextTablePrinter {
    fn print(&self, data: &JsonTable) -> GenericResult<()> {
        let mut table = PlainTextTable::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // header row
        table.set_header(Row::from(match &data.headers {
            TableHeader::NamedFields { fields } => fields
                .iter()
                .map(|f| Cell::new(f).fg(Color::Blue))
                .collect(),
            TableHeader::SingleUnnamedColumn => vec![Cell::new("value")],
        }));

        // build colorize map
        let colorize: HashMap<usize, Vec<&ColorizeSpec>> = match &data.headers {
            TableHeader::NamedFields { fields } => {
                let mut res: HashMap<usize, Vec<&ColorizeSpec>> = HashMap::new();
                for c in self.colorize.iter() {
                    if let Some(index) = fields.iter().position(|f| c.field == *f) {
                        res.entry(index).or_insert(Vec::new()).push(c)
                    }
                }
                res
            }
            _ => HashMap::new(),
        };

        // data rows
        for value in &data.values {
            let mut row = Row::new();
            for (_, element) in value.iter().enumerate() {
                let formatted = pprint_table_cell(element)?;
                let formatted = formatted.as_str();
                let cell = Cell::new(formatted);
                // TODO: compatible with colorspec
                // let cell = match colorize.get(&idx) {
                //     Some(styles) => match styles.iter().find(|s| s.value == *formatted) {
                //         Some(style) => cell.style_spec(style.style.as_str()),
                //         None => cell,
                //     },
                //     _ => cell,
                // };
                row.add_cell(cell);
            }
            table.add_row(row);
        }

        match &self.format {
            PlainTextTableFormat::Default => {
                table
                .set_corners('┌', '┐', '└', '┘')
                .set_intersections('├','┤','├','┤','┬','┼','┼','┴')
                .set_lines('─', '─', '│')
                .set_borders('│', '│', '─', '─');

            },
            PlainTextTableFormat::Markdown => {
                table
                .set_corners(' ',' ',' ',' ')
                .set_intersections('|', '|', '|', '|', ' ', '|', '|', ' ')
                .set_lines('-', '-', '|')
                .set_borders('|', '|', ' ', ' ');
            }
        }

        println!("{}", table);

        Ok(())
    }
}

pub struct HtmlTablePrinter {
    format: HtmlTableFormat,
}

impl HtmlTablePrinter {
    pub fn new(format: HtmlTableFormat) -> Self {
        Self { format }
    }
}

const BOOTSTRAP_CDN: &str = r#"
    <link
      rel="stylesheet"
      href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css"
      integrity="sha384-JcKb8q3iqJ61gNV9KGb8thSsNjpSL0n8PARn9HuZOnIxN0hoP+VmmDGMN5t9UJ0Z"
      crossorigin="anonymous">
"#;

impl Printer for HtmlTablePrinter {
    fn print(&self, data: &JsonTable) -> GenericResult<()> {
        let mut result = String::new();

        match self.format {
            HtmlTableFormat::Raw => result.push_str("<table>"),
            HtmlTableFormat::Styled => {
                result.push_str(BOOTSTRAP_CDN);
                result.push_str(r#"<table class="table table-bordered table-hover">"#)
            }
        }

        // header
        result.push_str("<tr>");
        match &data.headers {
            TableHeader::NamedFields { fields } => {
                for field in fields {
                    result.push_str(format!("<th>{}</th>", field).as_str())
                }
            }
            TableHeader::SingleUnnamedColumn => result.push_str("<th>Value</th>"),
        }
        result.push_str("</tr>");

        // rows
        for row in &data.values {
            result.push_str("<tr>");
            for element in row {
                let formatted = pprint_table_cell(element)?;
                let formatted = formatted.as_str();
                result.push_str(format!("<td><pre>{}</pre></td>", formatted).as_str())
            }
            result.push_str("</tr>");
        }

        result.push_str("</table>");

        println!("{}", result);

        Ok(())
    }
}
