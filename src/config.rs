use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    source_dir: String,
    target_dir: String,
}

impl Config {
    pub fn open(file_path: &str) -> Self {
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open config file '{}': {}", file_path, e);
                std::process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        Self::parse(reader)
    }

    fn parse(reader: impl BufRead) -> Self {
        let mut map = HashMap::new();
        let mut lines = reader.lines();
        loop {
            let line = match lines.next() {
                Some(v) => v.unwrap(),
                None => break,
            };
            if line.starts_with('#') {
                // skip comments
                continue;
            }
            let pair: Vec<_> = line.splitn(2, '=').collect();
            if pair.len() == 2 {
                let key = pair[0].trim();
                let value = pair[1].trim();
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

#[test]
fn test_config_open() {
    let input = "
source_dir=A
target_dir=B
";
    assert_eq!(
        Config::parse(BufReader::new(input.as_bytes())),
        Config {
            source_dir: "A".to_string(),
            target_dir: "B".to_string()
        }
    );
}

#[test]
fn test_config_whitespaces() {
    let input = "
  source_dir =A

    target_dir = BB C
";
    assert_eq!(
        Config::parse(BufReader::new(input.as_bytes())),
        Config {
            source_dir: "A".to_string(),
            target_dir: "BB C".to_string()
        }
    );
}

#[test]
fn test_config_tabs() {
    let input = "
source_dir=	A
target_dir  = B
";
    assert_eq!(
        Config::parse(BufReader::new(input.as_bytes())),
        Config {
            source_dir: "A".to_string(),
            target_dir: "B".to_string()
        }
    );
}

#[test]
fn test_config_comments() {
    let input = r#"
# Comment 1
source_dir = A

# Comment 2
target_dir = B
"#;
    assert_eq!(
        Config::parse(BufReader::new(input.as_bytes())),
        Config {
            source_dir: "A".to_string(),
            target_dir: "B".to_string()
        }
    );
}
