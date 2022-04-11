#[macro_use]
extern crate prettytable;

use std::collections::HashMap;
use clap::Parser;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{env, thread};
use std::fmt::format;
use std::fs::File;
use std::io::BufReader;
use std::process::Stdio;
use std::string::String;

mod utils;

use crate::utils::shrust::{ExecResult, Shell, ShellIO};
use crate::utils::run_script;
use crate::utils::conf::ByzerConf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    conf: Option<PathBuf>,
}


fn main() {
    let cli = Cli::parse();

    let mut config_path = ".mlsql.config";
    if let Some(_config_path) = cli.conf.as_deref() {
        config_path = _config_path.to_str().unwrap();
        println!("Conf file: {:?}", config_path)
    }

    let _byzer_home = env::current_exe().unwrap();

    let byzer_home = Path::new(&_byzer_home).parent().unwrap().parent().unwrap().to_str().unwrap();

    let config_path_opt = if Path::new(&config_path).exists() {
        Some(config_path.to_string())
    } else { None };

    let mut byzer_conf = ByzerConf::new(byzer_home.to_string(), config_path_opt);
    byzer_conf.build();

    let mut exec_c = std::process::Command::new(byzer_conf.build_java_command());
    exec_c.args(byzer_conf.byzer_command.as_slice());
    let mut pid = exec_c.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();

    let v: Vec<String> = Vec::new();
    let mut shell = Shell::new(v);

    shell.set_default(move |io, sh, s| {
        writeln!(io, "Executing....")?;
        let res = run_script(byzer_conf.engine_url.as_str(), s, byzer_conf.owner.as_str(), &byzer_conf.request_config);
        // writeln!(io, "Resp:\n {}", res);
        utils::print_as_table(res.as_str());
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());

    if pid.is_ok() {
        pid.unwrap().kill();
    };
}
