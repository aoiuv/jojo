use std::collections::HashMap;
use std::fs;
use std::process;

#[path = "define.rs"]
mod define;
#[path = "warn.rs"]
mod warn;

use define::{JO_CFG, PROTOCOL_BREAK, PROTOCOL_NEW_SEGMENT, PROTOCOL_SEP_ALIAS};

pub fn parse() -> HashMap<String, String> {
  let mut hashmap = HashMap::new();
  let footprint = fs::read_to_string(JO_CFG).unwrap_or_else(|_| {
    println!("{}", warn::warn_prefix(warn::error_lack_cfg()));
    process::exit(1);
  });

  let segments: Vec<&str> = footprint.split(PROTOCOL_NEW_SEGMENT).collect();

  for segment in segments {
    if segment.len() == 0 {
      continue;
    }
    let parts: Vec<&str> = segment.split(PROTOCOL_BREAK).collect();
    let alias: Vec<&str> = parts[0].split(PROTOCOL_SEP_ALIAS).collect();
    let target = parts[1];

    for a in alias {
      hashmap.insert(a.to_string(), target.to_string());
    }
  }

  hashmap
}
