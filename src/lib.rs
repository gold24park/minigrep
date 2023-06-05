use anygrep::{android_strgrep, minigrep};
use std::{env, error::Error, fs};
mod anygrep;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = match (config.read_xml_key, config.ignore_case) {
        (true, true) => android_strgrep::search_case_insensitive(&config.query, &contents),
        (true, false) => android_strgrep::search(&config.query, &contents),
        (false, true) => minigrep::search_case_insensitive(&config.query, &contents),
        (false, false) => minigrep::search(&config.query, &contents),
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub read_xml_key: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignore name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // GPT 추천
        let ignore_case = read_integer_env_var("IGNORE_CASE") == 1;
        let read_xml_key = read_integer_env_var("READ_XML_KEY") == 1;

        Ok(Config {
            query,
            file_path,
            ignore_case,
            read_xml_key,
        })
    }
}

fn read_integer_env_var(key: &str) -> i32 {
    env::var(key)
        .map(|value| value.parse::<i32>())
        .unwrap_or(Ok(0))
        .unwrap_or(0)
}
