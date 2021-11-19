use std::collections::HashMap;
use std::fs;
use std::process;

#[path = "define.rs"]
mod define;
#[path = "warn.rs"]
mod warn;

use define::{JO_CFG, SEP_BREAK, SEP_SEGMENT, SEP_ALIAS};

pub type Context = HashMap<String, String>;

// parse cfg file to hashmap
pub fn parse() -> Context {
  let mut hashmap = HashMap::new();

  let footprint = fs::read_to_string(JO_CFG).unwrap_or_else(|_| {
    println!("{}", warn::warn_prefix(warn::error_lack_cfg()));
    process::exit(1);
  });

  let segs: Vec<&str> = footprint.split(SEP_SEGMENT).collect();

  for s in segs {
    if s.len() == 0 {
      continue;
    }
    let parts: Vec<&str> = s.split(SEP_BREAK).collect();
    let keys: Vec<&str> = parts[0].split(SEP_ALIAS).collect();
    let target = parts[1];

    for k in keys {
      hashmap.insert(k.to_string(), target.to_string());
    }
  }

  hashmap
}

// update cfg
pub fn update(ctx: &mut Context, key: String, val: String) {
  ctx.insert(key, val);
}

pub fn serialize(ctx: Context) {

}
// pub fn erase() -> Result<Context, String> {}
