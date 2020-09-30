extern crate docopt;
use serde::{Serialize, Deserialize};

use docopt::Docopt;

#[derive(Serialize, Deserialize, Debug)]
struct Args {
    arg_file: Option<String>,
    flag_bytes: bool,
    flag_chars: bool,
    flag_lines: bool,
    flag_words: bool,
    flag_max_line_length: bool,
}

static USAGE: &'static str = "
Usage: wc [options] [<file>]

Options:
    -c, --bytes  print the byte counts
    -m, --chars  print the character counts
    -l, --lines  print the newline counts
    -w, --words  print the word counts
    -L, --max-line-length  print the length of the longest line
    -h, --help  display this help and exit
    -v, --version  output version information and exit
";

pub fn main() {
  println!("## Running Day 4 ##");
  let args: Args = Docopt::new(USAGE)
                    .and_then(|d| d.deserialize())
                    .unwrap_or_else(|e| e.exit());

  println!("Args: {:?}", args);

  println!("Counting stuff in '{}'", args.arg_file.unwrap_or("std in".to_string()));
  if args.flag_bytes {
    println!("Counting bytes");
  }
  if args.flag_chars {
    println!("Counting chars");
  }

}