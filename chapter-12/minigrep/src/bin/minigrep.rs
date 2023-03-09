use std::{env, process};

use minigrep::Config;

// run with cargo run -q --bin minigrep -- to poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
