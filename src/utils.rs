use std::collections::HashMap;
use stringreader::StringReader;
use reqwest;
use serde::de::Unexpected::Option;
use serde_json::{json, Value};

mod printer;
mod reader;
mod table_printer;
pub mod shrust;
pub mod conf;

use crate::utils::printer::{ColorizeSpec, HtmlTableFormat, JsonTable, PlainTextTableFormat, PlainTextTablePrinter, Printer, TableFormat, TableHeader};
use crate::utils::reader::{OneShotValueReader, ValueReader};

pub fn array_to_map<'a>(array: &'a [&str]) -> HashMap<&'a str, &'a str> {
    let mut element_map = HashMap::new();
    for x in (0..array.len()).step_by(2) {
        element_map.insert(array[x], array[x + 1]);
    }
    element_map
}

pub fn map_to_array<'a>(map: HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut elements = vec![];
    for (k, v) in map.into_iter() {
        elements.push(k);
        elements.push(v);
    }
    elements
}

pub fn run_script(endpoint: &str, sql: &str, owner: &str, config: &HashMap<String, String>) -> String {
    let client = reqwest::blocking::Client::new();
    let mut params = HashMap::new();
    params.insert("sql", sql);
    params.insert("owner", owner);
    params.insert("outputSize", "50");

    for (k, v) in config {
        params.insert((*k).as_str(), (*v).as_str());
    }

    let resp = client.post(endpoint).form(&params).send();
    let content = match resp {
        Ok(item) => item.text().unwrap(),
        Err(e) => format!("Fail to execute caused by {:?}", e.to_string())
    };
    content
}

pub fn print_as_table(data: &str) {
    let mut str_reader = StringReader::new(data);
    let mut onshot_reader = OneShotValueReader::new(str_reader);
    let newdata = match onshot_reader.read_value(Some(100)) {
        Ok(v) => v,
        Err(e) => {
            let temp_v = json!({
                "message":data
            });
            temp_v
        }
    };
    let table = JsonTable::new(None, &newdata);
    PlainTextTablePrinter::new(vec![], PlainTextTableFormat::Default).print(&table).unwrap();
}

