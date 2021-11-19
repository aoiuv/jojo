#![allow(dead_code)]

use std::env;
use std::process;

mod cfg;
mod dispatch;
mod jo;
mod warn;
mod define;

use dispatch::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let jo_cfg = cfg::Config::new(&args).unwrap_or_else(|err| {
        println!("{}", warn::warn_prefix(err));
        process::exit(1);
    });
    let mut ctx = jo::parse();

    dispatch(&mut ctx, &jo_cfg.action, &jo_cfg.params);
}
