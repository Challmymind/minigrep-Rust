use std::{env, process};
use minigrep::parser;

fn main() {
    let args = env::args();
    
    let results = parser::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Error occured, error text : {err} \n");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(results) {
        eprintln!("Error occured, error text : {e} \n");
        process::exit(1);
    }

}


