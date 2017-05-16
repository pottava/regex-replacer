extern crate regex;

use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::path::Path;
use std::env;
use regex::Regex;

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let f = File::open(path)?;
    let buf = BufReader::new(&f);
    Ok(buf.lines()
           .map(|l| l.unwrap_or(String::from("---")))
           .collect())
}

fn save_as<P: AsRef<Path>>(path: P, lines: &Vec<String>) -> Result<(), std::io::Error> {
    let mut f = File::create(path)?;
    for line in lines {
        f.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn main() {
    let delimiter = env::var("DELIMITER").unwrap_or(String::from(","));
    let replace_delimiter = env::var("REPLACE_DELIMITER").unwrap_or(String::from("="));
    let files = env::var("FILES").unwrap_or(String::from(""));
    let replaces = env::var("REPLACE").unwrap_or(String::from(""));

    for path in files.split(&delimiter) {
        match read_file(&path) {
            Ok(lines) => {
                let mut result: Vec<String> = vec![];
                let line_feed = String::from("\n");

                for line in lines {
                    let mut replaced = String::from(line);
                    for item in replaces.split(&delimiter) {
                        let mut iter = item.split(&replace_delimiter);
                        let from = iter.next().unwrap();
                        let to = iter.next().unwrap();
                        if let Ok(re) = Regex::new(from) {
                            replaced = String::from(re.replace_all(replaced.as_str(), to));
                        }
                    }
                    replaced.push_str(&line_feed);
                    result.push(replaced);
                }
                match save_as(&path, &result) {
                    Ok(_) => println!("Replaced successfully: {}", &path),
                    Err(err) => println!("Replace failed: {}", err),
                }
            }
            Err(err) => println!("Replace failed: {}", err),
        }
    }
}
