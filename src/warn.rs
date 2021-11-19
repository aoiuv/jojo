pub fn warn_prefix(msg: String) -> String {
  format!("[JOJO Warning] {}", msg)
}

pub fn error_lack_params(action: &String) -> String {
  format!("need params of action `{}`", action)
}

pub fn error_lack_action() -> String {
  "need action".to_string()
}
