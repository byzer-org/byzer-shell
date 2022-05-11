use reqwest;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::option::Option;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::thread::sleep;
use std::time;
use stringreader::StringReader;

pub mod conf;
mod printer;
mod reader;
pub mod shrust;
mod table_printer;
pub mod progress_bar;

use crate::utils::conf::ByzerConf;
use crate::utils::printer::{
    JsonTable, PlainTextTableFormat, PlainTextTablePrinter, Printer,
};
use crate::utils::reader::{OneShotValueReader, ValueReader};
use crate::utils::shrust::EditHelper;
use crate::utils::progress_bar::ExecutingProgressBar;


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

pub fn run_script(
    endpoint: &str,
    sql: &str,
    owner: &str,
    config: &HashMap<String, String>,
) -> String {
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
        Err(e) => format!("Fail to execute caused by {:?}", e.to_string()),
    };
    content
}

pub fn print_as_table(data: &str) {
    let mut str_reader = StringReader::new(data);
    let mut onshot_reader = OneShotValueReader::new(str_reader);
    let newdata = match onshot_reader.read_value(Some(100)) {
        Ok(v) => v,
        Err(e) => {
            let temp_v = json!({ "message": data });
            temp_v
        }
    };
    let table = JsonTable::new(None, &newdata);
    PlainTextTablePrinter::new(vec![], PlainTextTableFormat::Default)
        .print(&table)
        .unwrap();
}

pub fn run_loop<F>(func: F)
    where
        F: Fn(&str),
{
    let mut rl = Editor::new();
    let edit_helper = EditHelper::new();
    rl.set_helper(Some(edit_helper));
    let mut prompt = ">> ";
    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => func(&line),
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

pub fn show_version(byzer_conf: &ByzerConf) -> Option<String> {
    let version_info_query = "!show version;";
    let res = run_script(
        byzer_conf.engine_url.as_str(),
        version_info_query,
        byzer_conf.owner.as_str(),
        &byzer_conf.request_config,
    );
    if res.starts_with("Fail to execute caused by") {
        None
    } else {
        Some(res)
    }
}


pub fn print_pretty_header(byzer_conf: &ByzerConf) {
    //println!("Byzer-lang interpreter is starting...\n");
    let mut count = 0;
    let max_count = 30;
    let mut pb = ExecutingProgressBar::new();
    let monitor_handler = pb.start_monitor("Byzer-lang interpreter is starting...".to_string());
    while show_version(byzer_conf).is_none() && count < max_count {
        sleep(time::Duration::from_secs(1));
        count += 1
    }

    if count == max_count {
        pb.send_finish_signal(false);
        panic!("Fail to start byzer-lang interpreter")
    }

    pb.send_finish_signal(true);
    monitor_handler.join().unwrap();
    
    print_logo();

    let res = show_version(byzer_conf).unwrap();
    let version: Value = serde_json::from_str(&res).unwrap();
    println!("\n\nversion: {:?}", version[0]["version"].as_str().unwrap());
    println!("buildBy: {:?}", version[0]["buildBy"].as_str().unwrap());
    println!("date: {:?}", version[0]["date"].as_str().unwrap());
    println!(
        "srcChecksum: {:?}",
        version[0]["srcChecksum"].as_str().unwrap()
    );
    println!("revision: {:?}", version[0]["revision"].as_str().unwrap());
    println!("branch: {:?}", version[0]["branch"].as_str().unwrap());
    println!("url: {:?}", version[0]["url"].as_str().unwrap());
    println!("core: {:?}", version[0]["core"].as_str().unwrap());
    println!("\nType \"CTRL-C\" or \"CTRL-D\" to exit the program.\n");
}

pub fn print_logo() {
    println!(" _                                                 _              _   _ ");
    println!("| |__    _   _   ____   ___   _ __           ___  | |__     ___  | | | |");
    println!("| '_ \\  | | | | |_  /  / _ \\ | '__|  _____  / __| | '_ \\   / _ \\ | | | |");
    println!("| |_) | | |_| |  / /  |  __/ | |    |_____| \\__ \\ | | | | |  __/ | | | |");
    println!("|_.__/   \\__/ | /___|  \\___| |_|            |___/ |_| |_|  \\___| |_| |_|");
    println!("         |___/");
}
