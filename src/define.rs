use std::env;

pub fn get_sys_install_path() -> String {
  let usr = env::var("USER").unwrap();
  let mut install_path = String::from("/Users/");

  install_path.push_str(&usr);
  install_path.push_str("/jojo/cfg.jo");

  install_path
}

pub const SEP_SEGMENT: &str = "\n\n";
pub const SEP_BREAK: &str = "\n";
pub const SEP_UNIT : &str = ":";
