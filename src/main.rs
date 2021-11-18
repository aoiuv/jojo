// cargo run -- --config x=1
use std::env;
use std::process;

mod jo;
// use clap::{App, Arg, SubCommand};

fn main() {
    // let matches = App::new("JOJO")
    //     .version("1.0")
    //     .author("Suri Y. <yanguangjie@bytedance.com>")
    //     .about("Convenient command line path shuttle tool")
    //     .get_matches();

    // println!("{:?}", matches)
    let args: Vec<String> = env::args().collect();
    // let config = parse_config(&args);
    let config = jo::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = jo::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
