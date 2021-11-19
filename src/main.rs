use std::collections::HashMap;
use std::env;
use std::process;

mod jo;
mod warn;

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

fn dispatch(ctx: HashMap<String, String>, action: &Action, params: &String) {
    match action {
        Action::Register => {}
        Action::UnRegister => {}
        Action::List => {
            for (k, v) in ctx.iter() {
                println!("{:<10} => {:<2}", k, v);
            }
        }
        Action::Expand => {
            let result = ctx.get(params);
            match result {
                Some(v) => print!("{}", v),
                None => println!("{}", warn::warn_prefix(warn::error_no_register(params))),
            }
        }
        Action::Clean => {}
    }
}

fn get_params(action: &String, args: &[String]) -> Result<String, String> {
    if args.len() < 3 {
        return Err(warn::error_lack_params(&action));
    }
    Ok(args[2].clone())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(warn::error_lack_action());
        }
        let action = args[1].clone();

        match action.as_str() {
            "register" | "r" => {
                let params = get_params(&action, args)?;
                Ok(Config {
                    action: Action::Register,
                    params,
                })
            }
            "unregister" | "R" => {
                let params = get_params(&action, args)?;
                Ok(Config {
                    action: Action::UnRegister,
                    params,
                })
            }
            "expand" | "e" => {
                let params = get_params(&action, args)?;
                Ok(Config {
                    action: Action::Expand,
                    params,
                })
            }
            "list" | "l" => Ok(Config {
                action: Action::List,
                params: String::from(""),
            }),
            "clean" => Ok(Config {
                action: Action::Clean,
                params: String::from(""),
            }),
            _ => Err(warn::error_invalid_action(&action)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", warn::warn_prefix(err));
        process::exit(1);
    });
    let ctx = jo::parse();

    // println!("Context: {:?}", ctx);
    dispatch(ctx, &cfg.action, &cfg.params);

    // println!("Config: {:?}", cfg);
}
