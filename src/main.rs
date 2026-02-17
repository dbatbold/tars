use config::Config;
use std::env;

mod config;

fn main() {
    let mut conf = Config::default();

    parse_options(&mut conf);

    println!("{:?}", conf);
}

fn parse_options(conf: &mut Config) {
    let mut opt = String::new();
    let mut config_path = String::from("tars.config");
    for arg in env::args().skip(1) {
        match opt.as_str() {
            "-c" => {
                config_path = arg.to_string();
                opt.clear();
            }
            _ => (),
        }
        match arg.as_str() {
            "-c" => opt = arg.clone(),
            "-h" => print_help(),
            _ => (),
        }
    }
    *conf = Config::parse(&config_path);
}

fn print_help() {
    eprintln!(
        r#"tars
    -h              Print help
    -c <file>       Configation file path (default tars.config)
"#
    );
}
