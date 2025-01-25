use core::{f64, str};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{env, error::Error, fs, process};

struct Config {
    dict: String,
    file: String,
}

impl Config {
    fn new(input: Vec<String>) -> Option<Config> {
        if input.len() != 2 {
            None
        } else {
            Some(Config {
                file: input[0].clone(),
                dict: input[1].clone(),
            })
        }
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let words = fs::read_to_string(&config.dict)?;

    let n = words.lines().count();
    let m =
        ((n as f64) * f64::ln(0.05_f64) / (-f64::consts::LN_2 * f64::consts::LN_2)).ceil() as usize;
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

    let words = fs::read_to_string(&config.file)?;
    let words: Vec<&str> = words.split_ascii_whitespace().collect();
    for word in words {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        let word_hash = hasher.finish();

        for i in 0..k {
            let combined_hash = word_hash.wrapping_add((i as u64) * 0x9e3779b9);
            let index = (combined_hash % m as u64) as usize;
            if !bit_array[index] {
                println!("{} is misspelled", word);
                break;
            }
        }
    }
    Ok(())
}
fn main() {
    let args = Config::new(env::args().skip(1).collect::<Vec<String>>()).unwrap_or_else(|| {
        println!("Invalid arguments. \n Usage: kk-spell <file-to-check> <dictionary-file>");
        process::exit(1);
    });

    run(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });
}
