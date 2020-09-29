extern crate csv;
use csv::{Writer, ReaderBuilder};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
  title: String,
  bad_guy: String,
  pub_year: usize,
}

/// CSV
pub fn main() {
  let dollar_films = vec![
    ("A fistful of $$$", "Rojo", 1964),
    ("For a few $$$ more", "El indio", 1965),
    ("The good / bad / ugly", "Tuco", 1966)
  ];
  let path = "day3_westerners.csv";
  let mut writer = Writer::from_path(path).unwrap();
  // writer.write_record(&["title, bad_guy, pub_year"]).expect("Whoops");
  for row in dollar_films {
    writer.serialize(row).expect("CSV writer error");
  }

  let movie = Movie {
    title: "Hang em high".to_string(),
    bad_guy: "Wilson".to_string(),
    pub_year: 1968
  };

  writer.serialize(movie).expect("CSV writer error 2");
  writer.flush().expect("Flush error");

  ///////////////////////
  let mut rb = ReaderBuilder::new();
  rb.has_headers(false);

  let mut reader = rb.from_path(path).unwrap();
  for row in reader.deserialize() {
    let rowa: Movie = row.unwrap();
    println!("{:?}", rowa);
  }
}