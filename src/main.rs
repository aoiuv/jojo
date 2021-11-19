use std::collections::HashMap;
use std::env;
use std::process;

mod jo;
mod warn;

type TKeyTarget = Option<[Option<String>; 2]>;

#[derive(Debug)]
pub struct Config {
    pub action: Action,
    pub params: TKeyTarget,
}

#[derive(Debug)]
pub enum Action {
    Register,
    UnRegister,
    List,
    Expand,
    Clean,
}

fn dispatch(ctx: &mut jo::Context, action: &Action, params: &TKeyTarget) {
    match action {
        Action::Register => {
            let [_k, _v] = &params.as_ref().unwrap();
            let k = _k.as_ref().unwrap();
            let v = _v.as_ref().unwrap();

            jo::update(ctx, k.to_string(), v.to_string());
        }
        Action::UnRegister => {}
        Action::List => {
            for (k, v) in ctx.iter() {
                println!("{:<10} => {:<2}", k, v);
            }
        }
        Action::Expand => {
            let _params = &params.as_ref().unwrap();
            let k = _params[0].as_ref().unwrap();
            let result = ctx.get(k);

            match result {
                Some(v) => print!("{}", v),
                None => println!("{}", warn::warn_prefix(warn::error_no_register(&k))),
            }
        }
        Action::Clean => {}
    }
}

fn get_params<'a>(action: &String, args: &'a [String]) -> Result<&'a [String], String> {
    if args.len() < 3 {
        return Err(warn::error_lack_params(&action));
    }
    Ok(&args[2..])
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(warn::error_lack_action());
        }
        let action = args[1].clone();

        match action.as_str() {
            "register" | "r" => {
                let args = get_params(&action, args)?;
                let k = &args[0];
                let v = &args[1];

                Ok(Config {
                    action: Action::Register,
                    params: Some([Some(k.to_string()), Some(v.to_string())]),
                })
            }
            "unregister" | "R" => {
                let args = get_params(&action, args)?;
                let k = &args[0];

                Ok(Config {
                    action: Action::UnRegister,
                    params: Some([Some(k.to_string()), None]),
                })
            }
            "expand" | "e" => {
                let args = get_params(&action, args)?;
                let k = &args[0];

                Ok(Config {
                    action: Action::Expand,
                    params: Some([Some(k.to_string()), None]),
                })
            }
            "list" | "l" => Ok(Config {
                action: Action::List,
                params: None,
            }),
            "clean" => Ok(Config {
                action: Action::Clean,
                params: None,
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
    let mut ctx = jo::parse();

    dispatch(&mut ctx, &cfg.action, &cfg.params);
}
