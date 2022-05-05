use crate::utils::printer::{HtmlTableFormat, PlainTextTableFormat, TableFormat};
use std::error::Error;
use std::io;
use std::str::FromStr;

impl FromStr for TableFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(TableFormat::PlainText(PlainTextTableFormat::Default)),
            "markdown" => Ok(TableFormat::PlainText(PlainTextTableFormat::Markdown)),
            "html" => Ok(TableFormat::Html(HtmlTableFormat::Styled)),
            "html-raw" => Ok(TableFormat::Html(HtmlTableFormat::Raw)),
            _ => Err(format!("unknown format: {}", s)),
        }
    }
}
