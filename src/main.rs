use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Book {
    #[serde(rename = "Last Name")]
    #[serde(deserialize_with = "csv::invalid_option")]
    last_name: Option<String>, // 1
    #[serde(rename = "First Name")]
    #[serde(deserialize_with = "csv::invalid_option")]
    first_name: Option<String>, // 2
    #[serde(rename = "Middle Name")]
    #[serde(deserialize_with = "csv::invalid_option")]
    middle_name: Option<String>, // 3
    #[serde(rename = "Title")]
    #[serde(deserialize_with = "csv::invalid_option")]
    title: Option<String>, // 4
    #[serde(rename = "Subtitle")]
    #[serde(deserialize_with = "csv::invalid_option")]
    subtitle: Option<String>, // 5
    #[serde(rename = "Language")]
    #[serde(deserialize_with = "csv::invalid_option")]
    language: Option<String>, // 6
    #[serde(rename = "Year")]
    #[serde(deserialize_with = "csv::invalid_option")]
    year: Option<u16>, // 7
    #[serde(rename = "Series")]
    #[serde(deserialize_with = "csv::invalid_option")]
    series: Option<String>, // 8
    #[serde(rename = "ID")]
    id: u64, // 9
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut x: Vec<Book> = Vec::new();
    let mut err_count = 0;
    let file = File::open("catalog.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .flexible(true)
        .from_reader(buf_reader);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        match result {
            Ok(book) => {
                x.push(book);
            }
            Err(_) => {
                err_count += 1;
            }
        }

        // println!("{:?}", record);
    }
    println!("Len: {}", x.len());
    println!("Errors: {}", err_count);

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}