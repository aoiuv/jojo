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

pub fn error_failed_to_update_cfg(err: String) -> String {
  format!("failed to update config file `cfg.jo` due to: {}", err)
}

pub fn error_no_register(key: &String) -> String {
  format!("alias `{}` not register", key)
}

pub fn error_invalid_action(action: &String) -> String {
  format!("invalid action `{}`", action)
}
