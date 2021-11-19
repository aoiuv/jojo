#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[path = "define.rs"]
mod define;
#[path = "warn.rs"]
mod warn;

use define::{JO_CFG, SEP_BREAK, SEP_SEGMENT, SEP_UNIT};

pub type Context = HashMap<String, String>;

fn get_cfg_path() -> PathBuf {
  let cur = Path::new("./");
  let cfg_path = Path::new(JO_CFG);
  cur.join(cfg_path)
}

// parse cfg file to hashmap
pub fn parse() -> Context {
  let mut hashmap = HashMap::new();
  let cfg_path = get_cfg_path();

  let footprint = fs::read_to_string(cfg_path).unwrap_or_else(|_| {
    println!("{}", warn::warn_prefix(warn::error_lack_cfg()));
    process::exit(1);
  });

  let segs: Vec<&str> = footprint.split(SEP_SEGMENT).collect();

  for s in segs {
    if s.len() == 0 {
      continue;
    }
    let parts: Vec<&str> = s.split(SEP_BREAK).collect();
    let keys: Vec<&str> = parts[0].split(SEP_UNIT).collect();
    let target = parts[1];

    for k in keys {
      hashmap.insert(k.to_string(), target.to_string());
    }
  }

  hashmap
}

pub fn serialize(ctx: &Context) {
  let mut addr: HashMap<String, Vec<&String>> = HashMap::new();

  for (k, v) in ctx.iter() {
    match addr.get(v) {
      Some(_keys) => {
        let mut keys = _keys.clone();

        keys.push(k);
        addr.insert(v.to_string(), keys);
      }
      None => {
        addr.insert(v.to_string(), vec![k]);
      }
    }
  }

  let mut cfg_text = String::from("");

  for (k, v) in addr.iter() {
    let keys: Vec<String> = v.to_vec().iter().map(|x| x.to_string()).collect();
    let header = keys.join(SEP_UNIT);
    let body = k;

    // compose cfg text by reverse logic
    cfg_text.push_str(&header);
    cfg_text.push_str(SEP_BREAK);
    cfg_text.push_str(&body);
    cfg_text.push_str(SEP_SEGMENT);
  }

  if let Err(err) = fs::write(JO_CFG, cfg_text.trim_end()) {
    println!(
      "{}",
      warn::warn_prefix(warn::error_failed_to_update_cfg(err.to_string()))
    );
    process::exit(1);
  }
}

pub fn update(ctx: &mut Context, key: String, val: String) {
  ctx.insert(key, val);
}

pub fn erase(ctx: &mut Context, key: &String) {
  ctx.remove(key);
}

pub fn erase_all(ctx: &mut Context) {
  ctx.clear();
}
