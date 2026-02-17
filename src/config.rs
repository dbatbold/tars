use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
pub struct Config {
    source_dir: String,
    target_dir: String,
}

impl Config {
    pub fn parse(file_path: &str) -> Self {
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open config file '{}': {}", file_path, e);
                std::process::exit(1);
            }
        };
        let mut map = HashMap::new();
        let mut reader = BufReader::new(file);
        loop {
            let mut line = String::new();
            if 0 == reader.read_line(&mut line).unwrap() {
                break;
            }
            if line.starts_with('#') {
                // skip comments
                continue;
            }
            let pair: Vec<_> = line.splitn(2, '=').collect();
            if pair.len() == 2 {
                let key = pair[0];
                let value = match pair[1].strip_suffix('\n') {
                    Some(v) => v,
                    None => pair[1],
                };
                map.insert(key.to_string(), value.to_string());
                continue;
            }
            if line.trim().len() == 0 {
                // skip whitespaces
                continue;
            }
            eprint!("invalid config: {}", line);
        }

        let mut config = Config::default();
        for (key, value) in map.iter() {
            match key.as_str() {
                "source_dir" => config.source_dir = value.clone(),
                "target_dir" => config.target_dir = value.clone(),
                &_ => eprintln!("unknown config: {}", key),
            }
        }
        config
    }
}
