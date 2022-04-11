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
use crate::utils::{array_to_map, map_to_array, run_script};

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

    let _mlsql_home = env::current_exe().unwrap();

    let mlsql_home = Path::new(&_mlsql_home).parent().unwrap().parent().unwrap().to_str().unwrap();
    let java_home = match env::var("JAVA_HOME") {
        Ok(v) => v,
        Err(_) => {
            let mut path_buf = PathBuf::new();
            path_buf.push(mlsql_home);
            path_buf.push("jdk8");
            let t_path = path_buf.as_path();
            if t_path.exists() {
                t_path.to_str().unwrap().to_string()
            } else {
                String::from("")
            }
        }
    };

    let mut mlsql_config = HashMap::new();

    if Path::new(&config_path).exists() {
        let lines = BufReader::new(File::open(Path::new(config_path)).unwrap()).lines();
        for _line in lines {
            if let Ok(line) = _line {
                let line1 = line.trim();
                if line1.starts_with("#") {
                    continue;
                }
                if line1 == "" {
                    continue;
                }

                let kv = line1.splitn(2, "=").into_iter().collect::<Vec<_>>();
                mlsql_config.insert(kv[0].trim().to_owned(), kv[1].trim().to_owned());
            }
        }
    };

    let mut xmx = String::from("");

    if let Some(item) = (&mlsql_config).get("engine.memory") {
        xmx = ["-Xmx", item.as_str()].concat();
    }

    let mut owner = "admin".to_string();

    if let Some(item) = (&mlsql_config).get("user.owner") {
        owner = item.to_owned()
    }

    let mut executable = String::from("java");

    let mut java_name = "java";

    let mut classpath_seperator = ":";

    if env::consts::OS == "windows" {
        java_name = "java.exe";
        classpath_seperator = ";";
    }

    if java_home.is_empty() {
        let buf = PathBuf::new().join(java_name).join("bin").join(java_name);
        executable = buf.as_path().to_str().unwrap().to_owned()
    }

    let main_lib = PathBuf::new().join(mlsql_home).join("main").join("*");
    let libs_lib = PathBuf::new().join(mlsql_home).join("libs").join("*");
    let plugin_lib = PathBuf::new().join(mlsql_home).join("plugin").join("*");
    let spark_lib = PathBuf::new().join(mlsql_home).join("spark").join("*");

    let data_path = PathBuf::new().join(".").join("data");

    let main_class = "streaming.core.StreamingApp";

    let default_config_array = ["-streaming.master", "local[*]",
        "-streaming.name", "Byzer-shell",
        "-streaming.rest", "true",
        "-streaming.thrift", "false",
        "-streaming.platform", "spark",
        "-streaming.spark.service", "true",
        "-streaming.job.cancel", "true",
        "-streaming.datalake.path", data_path.as_path().to_str().unwrap(),
        "-streaming.driver.port", "9003",
        "-streaming.plugin.clzznames", "tech.mlsql.plugins.ds.MLSQLExcelApp,tech.mlsql.plugins.shell.app.MLSQLShell,tech.mlsql.plugins.assert.app.MLSQLAssert",
        "-streaming.mlsql.script.owner", owner.as_str()
    ];

    let mut request_config = HashMap::new();
    let default_config = array_to_map(&default_config_array);
    let mut modified_default_config = HashMap::new();

    for (k, v) in &default_config {
        modified_default_config.insert(k.to_string(), v.to_string());
    }


    for (k, v) in &mlsql_config {
        if k.starts_with("engine.spark") || k.starts_with("engine.streaming") {
            if k == "engine.streaming.plugin.clzznames" {
                modified_default_config.insert("-streaming.plugin.clzznames".to_string(), format!("{}{}{}", default_config["-streaming.plugin.clzznames"], ",", v.to_string()));
            } else if k == "engine.streaming.platform_hooks" {
                modified_default_config.insert("-streaming.platform_hooks".to_string(), format!("{}{}{}", default_config["-streaming.platform_hooks"], ",", v.to_string()));
            } else {
                modified_default_config.insert(format!("{}{}", "-", k.trim_start_matches("engine.")), v.to_string());
            }
        }

        if k.starts_with("user.") {
            request_config.insert(k.trim_start_matches("user.").to_string(), v.to_string());
        }
    }

    let mut engine_url = mlsql_config.get("engine.url").map(|item| { item.as_str().trim_end_matches("/") }).unwrap_or("http://127.0.0.1:9003").to_string();
    engine_url.push_str("/run/script");

    let mut temp_temp_config = HashMap::new();

    for (k, v) in &modified_default_config {
        temp_temp_config.insert(k.as_str(), v.as_str());
    }

    let final_config = map_to_array(temp_temp_config);

    let classpath = format!("{}{}{}{}{}{}{}",
                            main_lib.as_path().to_str().unwrap(),
                            classpath_seperator,
                            libs_lib.as_path().to_str().unwrap(),
                            classpath_seperator,
                            plugin_lib.as_path().to_str().unwrap(),
                            classpath_seperator,
                            spark_lib.as_path().to_str().unwrap());

    let temp_command = &["-cp", classpath.as_str(), main_class];

    let mut command = [temp_command, final_config.as_slice()].concat::<&str>();
    let xmx_slice = &[xmx.as_str()];
    if !xmx.is_empty() {
        command = [xmx_slice, command.as_slice()].concat::<&str>();
    }

    println!("{:?}", command.as_slice());

    let final_command = command.into_iter().map(|item| { item.to_owned() }).collect::<Vec<String>>();


    let mut exec_c = std::process::Command::new(executable);
    exec_c.args(final_command.as_slice());
    let mut pid = exec_c.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();

    let v: Vec<String> = Vec::new();
    let mut shell = Shell::new(v);

    shell.set_default(move |io, sh, s| {
        writeln!(io, "Executing....")?;
        let res = run_script(engine_url.as_str(), s, owner.as_str(), &request_config);
        // writeln!(io, "Resp:\n {}", res);
        utils::print_as_table(res.as_str());
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());

    if pid.is_ok() {
        pid.unwrap().kill();
    };
}
