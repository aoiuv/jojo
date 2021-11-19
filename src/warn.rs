#![allow(dead_code)]

pub fn warn_prefix(msg: String) -> String {
  format!("[JOJO Warning] {}", msg)
}

pub fn error_lack_params(action: &String) -> String {
  format!("need params of action `{}`", action)
}

pub fn error_lack_action() -> String {
  "need action".to_string()
}

pub fn error_lack_cfg() -> String {
  "config file `cfg.jo` not exist".to_string()
}

pub fn error_invalid_action(action: &String) -> String {
  format!("invalid action `{}`", action)
}
