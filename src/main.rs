use std::env;
use std::process;
use std::error;

const JO_CFG: &str = "cfg.jo";

#[derive(Debug)]
pub struct Config {
    pub action: Action,
    pub param: String,
}

#[derive(Debug)]
pub enum Action {
    Register,
    UnRegister,
    List,
    Expand,
    Clean,
}

fn dispatch(action: Action) {}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("need jo action".to_string());
        }
        let action = args[1].clone();
        if args.len() < 3 {
            return Err("need jo param of action".to_string());
        }
        let param = args[2].clone();

        match action.as_str() {
            "register" | "r" => Ok(Config {
                action: Action::Register,
                param,
            }),
            "unregister" | "R" => Ok(Config {
                action: Action::UnRegister,
                param,
            }),
            "list" | "l" => Ok(Config {
                action: Action::List,
                param,
            }),
            "expand" | "e" => Ok(Config {
                action: Action::Expand,
                param,
            }),
            "clean" => Ok(Config {
                action: Action::Clean,
                param,
            }),
            _ => Err(format!("invalid action `{}`", action)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("[JOJO Error] {}", err);
        process::exit(1);
    });

    println!("Config: {:?}", cfg);
}
