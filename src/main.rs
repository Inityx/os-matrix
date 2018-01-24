extern crate num_traits;

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

fn print_add(m1: Matrix, m2: Matrix) { println!("{}", (m1 + m2).unwrap()); }
fn print_dot(m1: Matrix, m2: Matrix) { println!("{}", m1.dot(m2).unwrap()); }

fn print_transpose(matrix: Matrix) { println!("{}", matrix); }

fn print_dims(matrix: Matrix) {
    let (rows, cols) = matrix.dims();
    println!("{} {}", rows, cols);
}

fn print_means(matrix: Matrix) {
    println!(
        "{}",
        matrix
            .column_means()
            .iter()
            .map(NumberType::to_string)
            .collect::<Vec<String>>()
            .as_slice()
            .join("\t")
    );
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
        "dims"             => print_dims(matrix_1),
        "transpose"        => print_transpose(matrix_1),
        "mean"             => print_means(matrix_1),
        "add" | "multiply" => {
            let matrix_2 = matrix_2_option.expect("Matrix 2 not provided");
            match mode.as_str() {
                "add"      => print_add(matrix_1, matrix_2),
                "multiply" => print_dot(matrix_1, matrix_2),
                _          => unreachable!(),
            };
        },
        _ => usage(),
    };
}
