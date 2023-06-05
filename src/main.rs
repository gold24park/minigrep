use minigrep::{run, Config};
use std::{env, process};

fn main() {
    // iterator 자체를 넘겨서 iterator의 ownership을 env::args -> Config::build로 넘긴다.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // stderr
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
