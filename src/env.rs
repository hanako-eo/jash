use std::collections::HashMap;
use std::env;

pub struct Variables {
  base: HashMap<String, String>
}

impl Variables {
  pub fn empty() -> Self {
    Self {
      base: HashMap::new()
    }
  }

  pub fn new() -> Self {
    let mut vars = Self::empty();
    for (key, value) in env::vars_os() {
      vars.set(key.into_string().unwrap(), value.into_string().unwrap());
    }
    vars
  }

  pub fn contains_key(&self, key: String) -> bool {
    self.base.contains_key(&key)
  }

  pub fn get(&self, key: String) -> Option<&String> {
    self.base.get(&key)
  }

  pub fn set(&mut self, key: String, value: String) {
    self.base.insert(key.clone(), value.clone());
    env::set_var(key, value);
  }

  pub fn set_local(&mut self, key: String, value: String) {
    self.base.insert(key, value);
  }

  pub fn remove(&mut self, key: String) {
    self.base.remove(&key);
    env::remove_var(key);
  }

  pub fn len(&mut self) -> usize {
    self.base.len()
  }
}

pub fn get(key: &str) -> String {
  env::var(key).unwrap_or_default()
}
