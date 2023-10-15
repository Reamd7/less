use std::{path::{Path, PathBuf}, str::FromStr}; 
use lazy_static::lazy_static;
use regex::Regex;

pub struct AbstractFileManagerStruct;


impl AbstractFileManager for AbstractFileManagerStruct {}

pub trait AbstractFileManager {
  fn get_path(filename: &str) -> &str {
    let mut filename = filename;
    let j = filename.rfind('?');
    if j.is_some() {
      filename = &filename[0..j.unwrap()];
    }
    let mut j = filename.rfind("/");
    if j.is_none() {
      j = filename.rfind('\\');
    }
    if j.is_none() {
      return "";
    }
    &filename[0..j.unwrap() + 1]
  }

  fn try_append_extension(path: &str, ext: &str) -> String {
    lazy_static! {
      static ref NEED_APPEND_EXT: regex::Regex = Regex::new(
        r"(\.[a-z]*$)|([?;].*)$"
      ).expect("error parse regex");
    }
    if NEED_APPEND_EXT.is_match(path) {
      String::from(path)
    } else {
      let mut res = String::from(path);
      res.push_str(ext);
      res
    }
  }

  fn supports_sync() -> bool {
    false
  }

  fn always_make_paths_absolute() -> bool {
    false
  }

  fn is_path_absolute(filename: &str) -> bool {
    lazy_static! {
      static ref PATH_ABSOLUTE_REG: regex::Regex = Regex::new(
        r"^(?:[a-zA-Z-]+:|\/|\\|#)"
      ).expect("error parse regex");
    }
    PATH_ABSOLUTE_REG.is_match(filename)
  }

  // TODO: pull out / replace?
  fn join(base_path: &str, later_path: &str) -> String {
    if base_path.is_empty() {
      String::from(later_path)
    } else {
      let mut res = String::from(base_path);
      res.push_str(later_path);
      res
    }
  }

  fn extractUrlParts(url: &str, baseUrl: &str) {
    lazy_static! {
      static ref urlPartsRegex: regex::Regex = Regex::new(
        r"^((?:[a-zA-Z-]+:)?\/{2}(?:[^/?#]*\/)|([/\\]))?((?:[^/\\?#]*[/\\])*)([^/\\?#]*)([#?].*)?$"
      ).expect("error parse regex");
    }

    let urlParts = urlPartsRegex.captures(url).unwrap_or_else(|| panic!("Could not parse sheet href - '{url}'"));

  }
  
}
