extern crate num;

mod matrix;

use std::convert::From;
use std::env::args;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

const USAGE_TEXT: &'static str = "Usage: matrix operation [source1 [source2]]\n";
const STDIN_FILENAME: &'static str = "/dev/stdin";

type BoxedError = Box<std::error::Error>;
type Result<T> = std::result::Result<T, BoxedError>;
type NumberType = i32;
type Matrix = matrix::Matrix<NumberType>;

fn usage() -> ! {
    std::io::stderr().write(USAGE_TEXT.as_bytes()).unwrap();
    std::process::exit(1);
}

fn matrix_from_file<P: AsRef<Path>>(path: P) -> Result<Matrix> {
    let mut contents = String::new();
    File::open(path)
        .map_err(<BoxedError>::from)?
        .read_to_string(&mut contents)
        .map_err(<BoxedError>::from)?;

    contents.parse::<Matrix>()
}

fn vec_format(vector: &Vec<NumberType>) -> String {
    vector
        .iter()
        .map(NumberType::to_string)
        .collect::<Vec<String>>()
        .as_slice()
        .join("\t")
}

fn main() {
    let mode = match args().nth(1) { 
        Some(mode) => mode,
        None => usage(),
    };

    let matrix_1 = matrix_from_file(
        args().nth(2).unwrap_or(String::from(STDIN_FILENAME))
    ).expect("Error parsing matrix 1");

    let matrix_2_option = args().nth(3)
        .map(matrix_from_file)
        .map(|result| result.expect("Error parsing matrix 2"));

    match mode.as_str() {
        "dims"             => println!("{} {}", matrix_1.dims().0, matrix_1.dims().1),
        "transpose"        => println!("{}", matrix_1.transpose()),
        "mean"             => println!("{}", vec_format(&matrix_1.column_means())),
        "add" | "multiply" => {
            let matrix_2 = matrix_2_option.expect("Matrix 2 not provided");
            match mode.as_str() {
                "add"      => println!("{}", matrix_1.add(&matrix_2).unwrap()),
                "multiply" => println!("{}", matrix_1.dot(&matrix_2).unwrap()),
                _          => unreachable!(),
            };
        },
        _ => usage(),
    };
}
