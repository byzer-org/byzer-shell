#[macro_use]
extern crate prettytable;

use clap::Parser;

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::env;
use utils::print_pretty_header;

mod utils;

use crate::utils::conf::ByzerConf;
use crate::utils::progress_bar::ExecutingProgressBar;
use crate::utils::{run_loop, run_script};

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

    let byzer_home = Path::new(&_byzer_home)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    let config_path_opt = if Path::new(&config_path).exists() {
        Some(config_path.to_string())
    } else {
        None
    };

    let mut byzer_conf = ByzerConf::new(byzer_home.to_string(), config_path_opt);
    byzer_conf.build();

    let java_exec = byzer_conf.build_java_command();

    let mut exec_c = std::process::Command::new(java_exec);
    exec_c.args(byzer_conf.byzer_command.as_slice());
    let mut pid = exec_c.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();

    print_pretty_header(&byzer_conf);

    run_loop(move |s| {
        println!("{}", "\n");
        let mut pb = ExecutingProgressBar::new();
        let monitor_handler = pb.start_monitor();
        
        let res = run_script(
            byzer_conf.engine_url.as_str(),
            s,
            byzer_conf.owner.as_str(),
            &byzer_conf.request_config,
        );

        if res.starts_with("MLSQL Parser error") {
            pb.send_finish_signal(false);
        } else {
            pb.send_finish_signal(true);
        }

        monitor_handler.join().unwrap();

        utils::print_as_table(res.as_str());
    });

    if pid.is_ok() {
        pid.unwrap().kill();
    };
}
