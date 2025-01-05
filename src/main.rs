use std::{env, process};

enum Config {
    Build(String),
    Check { dictionary: String, file: String },
}

impl Config {
    fn new(mut input: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let first = input
            .next()
            .ok_or("Please provide at least 2 arguments.")
            .unwrap();
        match first.as_ref() {
            "--build" => Ok(Config::Build(
                input
                    .next()
                    .ok_or("Please provide a dictionary filename to build.")
                    .unwrap(),
            )),
            _ => Ok(Config::Check {
                file: first,
                dictionary: input
                    .next()
                    .ok_or("Please provide a file to check.")
                    .unwrap(),
            }),
        }
    }
}

fn main() {
    let args = Config::new(env::args().skip(1)).unwrap_or_else(|err| {
        println!("Invalid arguments: {}", err);
        process::exit(1);
    });
}
