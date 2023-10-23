use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", cfg.query);
    //println!("In file {}", cfg.file_path);

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(2);
    }
}
