use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1 as alphanumeric, multispace1 as multispace},
    sequence::{preceded, tuple},
    IResult,
};
use std::fs;

#[derive(Debug)]
struct Record {
    field1: String,
    field2: String,
    // Add more fields as per your CSV structure
}

fn parse_line(input: &str) -> IResult<&str, Record> {
    let (input, (field1, _, field2)) = tuple((alphanumeric, tag(","), alphanumeric))(input)?;
    Ok((input, Record { field1: field1.to_string(), field2: field2.to_string() }))
}

fn main() {
    let data = fs::read_to_string("src/path_to_your_file.csv").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();
    for line in lines {
        match parse_line(line) {
            Ok((_, record)) => println!("{:?}", record),
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
}