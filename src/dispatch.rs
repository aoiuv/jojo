#![allow(dead_code)]

#[path = "cfg.rs"]
mod cfg;
#[path = "jo.rs"]
mod jo;
#[path = "warn.rs"]
mod warn;

use super::cfg::{Action, TKeyTarget};

pub fn dispatch(ctx: &mut jo::Context, action: &Action, params: &TKeyTarget) {
  match action {
    Action::Register => {
      let [_k, _v] = &params.as_ref().unwrap();
      let k = _k.as_ref().unwrap();
      let v = _v.as_ref().unwrap();

      jo::update(ctx, k.to_string(), v.to_string());
      jo::serialize(ctx);
    }
    Action::UnRegister => {
      let [_k, _] = &params.as_ref().unwrap();
      let k = _k.as_ref().unwrap();

      jo::erase(ctx, &k);
      jo::serialize(ctx);
    }
    Action::Clean => {
      jo::erase_all(ctx);
      jo::serialize(ctx);
    }
    Action::List => {
      for (k, v) in ctx.iter() {
        println!("{:<10} => {:<2}", k, v);
      }
    }
    Action::Expand => {
      let [_k, _] = &params.as_ref().unwrap();
      let k = _k.as_ref().unwrap();
      let result = ctx.get(k);

      match result {
        Some(v) => print!("{}", v),
        None => println!("{}", warn::warn_prefix(warn::error_no_register(&k))),
      }
    }
  }
}
