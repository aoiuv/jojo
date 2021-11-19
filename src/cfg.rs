#![allow(dead_code)]

#[path = "warn.rs"]
mod warn;

pub type TKeyTarget = Option<[Option<String>; 2]>;

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
        if args.len() < 2 {
          return Err(warn::error_lack_params(&action));
        }
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
