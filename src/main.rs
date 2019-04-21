use docopt::Docopt;
use serde::Deserialize;
use regex::Regex;
use std::{str, thread};
const USAGE: &'static str = "
MD5 magic bruteforce.

Usage:
  md5_magic <pattern>
  md5_magic (-h | --help)
  md5_magic --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_pattern: String,
}

static ALLOWED_CHARS: &'static [&'static str] = &["A", "B", "C", "D", "E", "F", "G", "H",
"I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "i", "j",
"k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
"_", "-", "!", "#", "$", "%", "&" ,"*","+", "/", "=", "?", "^", "`", "{", "|", "}", "~"];

fn check(result: &str, pattern: &str, regex: &Regex){
  let clear = pattern.replace("{{RAND}}", &result);
  let subdigest = &format!("{:x}",&md5::compute(&clear))[0..10];
  if regex.is_match(subdigest){
    println!("{}:{}", clear, subdigest);
  }
}

fn generate_string(result: &str,  depth: u8, pattern: &str, start: usize, end: usize, regex: &Regex){
  if depth == 1{
    for i in start..end{
      check(&format!("{}{}",result, ALLOWED_CHARS[i]), &pattern, regex);
    }
    return;
  }
  if result.is_empty(){
    generate_string(result.clone(), depth - 1, pattern, start, end, regex);
  }
  for letter in ALLOWED_CHARS{
    generate_string(&format!("{}{}", result, letter), depth - 1, pattern, start, end, regex);
  }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let mut threads = Vec::new();
    let num = num_cpus::get();
    let slice_size = (ALLOWED_CHARS.len() / num) as usize;
    for i in 0..num{
      let start = i * slice_size;
      let mut end = start + slice_size;
      if i == num - 1{
        end = ALLOWED_CHARS.len();
      }
      let pattern = args.arg_pattern.clone();
      threads.push(thread::spawn(move || {
        generate_string("", 255, &pattern, start, end, &Regex::new(r"^0e\d+$").unwrap());
      }));
    }
    for thread in threads{
      let _res = thread.join();
    }
}
