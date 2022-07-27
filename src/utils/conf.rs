use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use crate::utils::{array_to_map, map_to_array};

pub struct ByzerConf {
    byzer_home: String,
    java_home: String,
    config_path: Option<String>,
    pub engine_url: String,
    pub request_config: HashMap<String, String>,
    pub byzer_command: Vec<String>,
    pub owner: String,
}

fn scan_port(port: u16) -> bool {
    match TcpStream::connect(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn available_port() -> u16 {
    let mut start_port = 9003;
    while scan_port(start_port) {
        start_port += 1
    };
    start_port
}

impl ByzerConf {
    pub fn new(byzer_home: String, config_path: Option<String>) -> ByzerConf {
        let java_home = match env::var("JAVA_HOME") {
            Ok(v) => v,
            Err(_) => {
                let mut path_buf = PathBuf::new();
                path_buf.push(&byzer_home);
                path_buf.push("jdk8");

                if env::consts::OS == "macos" {
                    path_buf.push("Contents");
                    path_buf.push("Home");
                }

                let t_path = path_buf.as_path();
                if t_path.exists() {
                    t_path.to_str().unwrap().to_string()
                } else {
                    String::from("")
                }
            }
        };

        let mut conf = ByzerConf {
            byzer_home,
            java_home,
            config_path,
            engine_url: String::from("http://127.0.0.1:9003"),
            request_config: HashMap::new(),
            byzer_command: vec![],
            owner: String::from("admin"),
        };
        conf
    }

    pub fn build_java_command(&mut self) -> String {
        let mut executable = String::from("java");

        let mut java_name = "java";

        let mut classpath_seperator = ":";

        if env::consts::OS == "windows" {
            java_name = "java.exe";
            classpath_seperator = ";";
        }

        if !self.java_home.is_empty() {
            let buf = PathBuf::new()
                .join(self.java_home.as_str())
                .join("bin")
                .join(java_name);
            executable = buf.as_path().to_str().unwrap().to_owned()
        };

        executable
    }


    pub fn build(&mut self) -> &ByzerConf {
        let mut mlsql_config = self.read_config_from_file();

        let mut xmx = String::from("");

        if let Some(item) = (&mlsql_config).get("engine.memory") {
            xmx = ["-Xmx", item.as_str()].concat();
        }

        if let Some(item) = (&mlsql_config).get("user.owner") {
            self.owner = item.to_owned()
        }

        let main_lib = PathBuf::new()
            .join(self.byzer_home.as_str())
            .join("main")
            .join("*");
        let libs_lib = PathBuf::new()
            .join(self.byzer_home.as_str())
            .join("libs")
            .join("*");
        let plugin_lib = PathBuf::new()
            .join(self.byzer_home.as_str())
            .join("plugin")
            .join("*");
        let spark_lib = PathBuf::new()
            .join(self.byzer_home.as_str())
            .join("spark")
            .join("*");

        let data_path = PathBuf::new().join(".").join("data");

        let main_class = "streaming.core.StreamingApp";

        let interpreter_port = available_port().to_string();

        let default_config_array = ["-streaming.master", "local[*]",
            "-streaming.name", "Byzer-shell",
            "-streaming.rest", "true",
            "-streaming.thrift", "false",
            "-streaming.platform", "spark",
            "-streaming.spark.service", "true",
            "-streaming.job.cancel", "true",
            "-streaming.datalake.path", data_path.as_path().to_str().unwrap(),
            "-streaming.driver.port", interpreter_port.as_str(),
            "-streaming.plugin.clzznames", "tech.mlsql.plugins.ds.MLSQLExcelApp,tech.mlsql.plugins.shell.app.MLSQLShell,tech.mlsql.plugins.assert.app.MLSQLAssert",
            "-streaming.mlsql.script.owner", self.owner.as_str()
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
                    modified_default_config.insert(
                        "-streaming.plugin.clzznames".to_string(),
                        format!(
                            "{}{}{}",
                            default_config["-streaming.plugin.clzznames"],
                            ",",
                            v.to_string()
                        ),
                    );
                } else if k == "engine.streaming.platform_hooks" {
                    modified_default_config.insert(
                        "-streaming.platform_hooks".to_string(),
                        format!(
                            "{}{}{}",
                            default_config["-streaming.platform_hooks"],
                            ",",
                            v.to_string()
                        ),
                    );
                } else {
                    modified_default_config.insert(
                        format!("{}{}", "-", k.trim_start_matches("engine.")),
                        v.to_string(),
                    );
                }
            }

            if k.starts_with("user.") {
                request_config.insert(k.trim_start_matches("user.").to_string(), v.to_string());
            }
        }

        let default_interpreter_address = format!("http://127.0.0.1:{}", interpreter_port);
        let mut engine_url = mlsql_config
            .get("engine.url")
            .map(|item| item.as_str().trim_end_matches("/"))
            .unwrap_or(default_interpreter_address.as_str())
            .to_string();
        engine_url.push_str("/run/script");

        self.engine_url = engine_url;

        self.request_config = request_config;

        let mut temp_temp_config = HashMap::new();

        for (k, v) in &modified_default_config {
            temp_temp_config.insert(k.as_str(), v.as_str());
        }

        let final_config = map_to_array(temp_temp_config);

        let mut classpath_seperator = ":";

        if env::consts::OS == "windows" {
            classpath_seperator = ";";
        }

        let classpath = format!(
            "{}{}{}{}{}{}{}",
            main_lib.as_path().to_str().unwrap(),
            classpath_seperator,
            libs_lib.as_path().to_str().unwrap(),
            classpath_seperator,
            plugin_lib.as_path().to_str().unwrap(),
            classpath_seperator,
            spark_lib.as_path().to_str().unwrap()
        );

        let temp_command = &["-cp", classpath.as_str(), main_class];

        let mut command = [temp_command, final_config.as_slice()].concat::<&str>();
        let xmx_slice = &[xmx.as_str()];
        if !xmx.is_empty() {
            command = [xmx_slice, command.as_slice()].concat::<&str>();
        }
        let final_command = command
            .into_iter()
            .map(|item| item.to_owned())
            .collect::<Vec<String>>();
        self.byzer_command = final_command;
        self
    }

    fn read_config_from_file(&mut self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        if self.config_path.is_none() {
            return config;
        }
        let p = self.config_path.as_ref().unwrap();

        let b_reader = BufReader::new(File::open(Path::new(p)).unwrap());

        let lines = b_reader.lines();
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
                config.insert(kv[0].trim().to_owned(), kv[1].trim().to_owned());
            }
        }
        config
    }
}
