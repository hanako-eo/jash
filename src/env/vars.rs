use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;

pub fn all() -> HashMap<String, String> {
  env::vars().collect()
}

pub fn get<S>(key: S) -> String
where
  S: AsRef<OsStr>
{
  env::var(key).unwrap_or_default()
}

pub fn set<S, S2>(key: S, value: S2)
where
  S: AsRef<OsStr>,
  S2: AsRef<OsStr>
{
  env::set_var(key, value);
}

pub fn contains<S>(key: S) -> bool
where
  S: AsRef<OsStr>
{
  env::var(key).is_ok()
}

pub fn remove<S>(key: S)
where
  S: AsRef<OsStr>
{
  env::remove_var(key);
}