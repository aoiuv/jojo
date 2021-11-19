use std::env;
use std::process;

const JO_CFG: &str = "cfg.jo";

#[derive(Debug)]
pub struct Config {
    pub action: Action,
    pub params: String,
}

#[derive(Debug)]
pub enum Action {
    Register,
    UnRegister,
    List,
    Expand,
    Clean,
}

fn warn(msg: String) -> String {
    format!("[JOJO Warning] {}", msg)
}

fn dispatch(action: &Action, params: &String) {}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("need action".to_string());
        }
        let action = args[1].clone();

        if args.len() < 3 {
            let lack_params = format!("need params of action `{}`", action);
            return Err(lack_params);
        }
        let params = args[2].clone();

        match action.as_str() {
            "register" | "r" => Ok(Config {
                action: Action::Register,
                params,
            }),
            "unregister" | "R" => Ok(Config {
                action: Action::UnRegister,
                params,
            }),
            "list" | "l" => Ok(Config {
                action: Action::List,
                params,
            }),
            "expand" | "e" => Ok(Config {
                action: Action::Expand,
                params,
            }),
            "clean" => Ok(Config {
                action: Action::Clean,
                params,
            }),
            _ => Err(format!("invalid action `{}`", action)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", warn(err));
        process::exit(1);
    });
    // dispatch action and it's params
    dispatch(&cfg.action, &cfg.params);

    println!("Config: {:?}", cfg);
}
