use core::{f64, str};
use std::{env, error::Error, fs, path::Path, process};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Build(path) => {
            let words = fs::read_to_string(path)?;

            let n = words.lines().count();
            let m = ((n as f64) * f64::ln(0.05_f64) / (-f64::consts::LN_2 * f64::consts::LN_2))
                .ceil() as usize;
            let k = (m as f64 / n as f64 * f64::ln(2.0f64)).round() as u32;

            let mut bit_array = vec![false; m];

            for word in words.lines().map(|l| l.trim()).map(|w| w.to_lowercase()) {
                let mut hasher = DefaultHasher::new();
                word.hash(&mut hasher);
                let word_hash = hasher.finish();

                for i in 0..k {
                    let combined_hash = word_hash.wrapping_add((i as u64) * 0x9e3779b9);
                    let index = (combined_hash % m as u64) as usize;
                    bit_array[index] = true;
                }
            }

            let contents = [
                format!("kkspell.{}.{}.", k, m).as_bytes(),
                &bit_array.iter().map(|a| *a as u8).collect::<Vec<u8>>(),
            ]
            .concat();
            let path = Path::new(path).with_extension("spell");
            fs::write(&path, contents)?;
            println!("Wrote data to {}", &path.display());
        }
        Config::Check { dictionary, file } => {
            if !dictionary.ends_with(".spell") {
                return Err("The dictionary file must have a .spell extension".into());
            }

            let bytes = fs::read(dictionary)?;
            if str::from_utf8(&bytes[0..=6])? != "kkspell" {
                return Err(format!("the file: {} is not a valid .spell file", dictionary).into());
            }

            // 2e == '.'
            let mut iter = bytes.split(|&b| b == 0x2e).skip(1).take(3);
            let k = str::from_utf8(iter.next().ok_or("parsing error")?)?.parse::<usize>()?;
            let m = str::from_utf8(iter.next().ok_or("parsing error")?)?.parse::<usize>()?;

            let bit_array = iter.next().ok_or("parsing error")?;
            println!("{:?}", &bit_array[0..50]);

            let words = fs::read_to_string(file)?;
            let words: Vec<&str> = words.split_ascii_whitespace().collect();
            for word in words {
                let mut hasher = DefaultHasher::new();
                word.hash(&mut hasher);
                let word_hash = hasher.finish();

                for i in 0..k {
                    let combined_hash = word_hash.wrapping_add((i as u64) * 0x9e3779b9);
                    let index = (combined_hash % m as u64) as usize;
                    if bit_array[index] == 0 {
                        println!("{} is misspelled", word);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
fn main() {
    let args = Config::new(env::args().skip(1)).unwrap_or_else(|err| {
        println!("Invalid arguments: {}", err);
        process::exit(1);
    });

    run(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });
}
