use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {

        println!("{}", err);

        process::exit(1);
    });
   
    println!("query {}", config.query);

    println!("filename {}", config.filename);

    if let Err(e) = minigrep::run(config) {

        println!("run err {}", e);

        process::exit(1);
    }
}
