#![allow(dead_code)]

use ::num_traits::cast::AsPrimitive;

use ::std::iter::Sum;
use ::std::ops::{Add, Mul, Div};
use ::std::str::FromStr;
use ::std::fmt;

#[derive(Debug)]
pub struct Error(String);

impl Error { fn new(string: &str) -> Error { Error(String::from(string)) } }

impl ::std::error::Error for Error {
    fn description(&self) -> &str { self.0.as_str() }
    fn cause(&self) -> Option<&::std::error::Error> { None }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Matrixable:
    Default +
    Copy +
    Mul<Output=Self> +
    Div<Output=Self> +
    Add<Output=Self> +
    Sum {}

impl<T:
    Default +
    Copy +
    Mul<Output=Self> +
    Div<Output=Self> +
    Add<Output=Self> +
    Sum
> Matrixable for T {}

#[derive(Default)]
pub struct Matrix<T>{
    num_rows: usize,
    num_cols: usize,
    data: Vec<T>,
}

impl<T: Matrixable> Matrix<T> {
    pub fn new() -> Self { Default::default() }
    pub fn from_1d(num_rows: usize, data: Vec<T>) -> Result<Self, Error> {
        let modulo = data.len() % num_rows;
        if modulo != 0 {
            return Err(Error::new(&format!(
                "Heterogeneous row length: {}%{}=={}!=0",
                data.len(), num_rows, modulo
            )));
        }
        Ok(Matrix {
            num_rows: num_rows,
            num_cols: data.len() / num_rows,
            data: data,
        })
    }
    pub fn from_2d(data: Vec<Vec<T>>) -> Result<Self, Error> {
        if data.is_empty() { return Ok(Self::new()); }

        let num_rows = data.len();
        let num_cols = data[0].len();

        if data.iter().any(|row| row.len() != num_cols) {
            return Err(Error::new("Heterogeneous row length"));
        }

        Ok(Matrix{
            num_rows: num_rows,
            num_cols: num_cols,
            data: data.concat(),
        })
    }
    
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn dims(&self) -> (usize, usize) { (self.num_rows, self.num_cols) }
    pub fn rows<'a>(&'a self) -> ::std::slice::Chunks<'a, T> {
        self.data.chunks(self.num_cols)
    }
    pub fn transpose(&self) -> Self {
        let mut new_data: Vec<T> = Vec::new();
        for col_index in 0..self.num_cols {
            for row in self.rows() {
                new_data.push(row[col_index].clone());
            }
        }
        Self {
            num_cols: self.num_rows,
            num_rows: self.num_cols,
            data: new_data,
        }
    }
    pub fn dot(self, rhs: Self) -> Result<Self, Error> {
        if self.num_cols != rhs.num_rows {
            return Err(Error::new(&format!(
                "Incompatible dimensions ({} != {})",
                self.num_cols, rhs.num_rows,
            )));
        }
        let mut new_data = Vec::new();
        let rhs_transpose = rhs.transpose();

        for m1_row in self.rows() {
            for m2_row in rhs_transpose.rows() {
                new_data.push(
                    m1_row.iter().cloned()
                        .zip(m2_row.iter().cloned())
                        .map(|(a, b)| a * b)
                        .sum::<T>()
                );
            }
        }
        
        Ok(Self {
            num_rows: self.num_rows,
            num_cols: rhs.num_cols,
            data: new_data
        })
    }
    
}

impl<T: Matrixable + AsPrimitive<isize>> Matrix<T> where isize: AsPrimitive<T> {
    pub fn column_means(&self) -> Vec<T> {
        self
            .transpose()
            .rows()
            .map(|row| {
                let sum: isize = row
                    .iter()
                    .cloned()
                    .map(AsPrimitive::<isize>::as_)
                    .sum();
                AsPrimitive::<T>::as_(sum / row.len() as isize)
            })
            .collect::<Vec<T>>()
    }
}

impl<T: Matrixable> Add for Matrix<T> {
    type Output=Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.num_rows != rhs.num_rows || self.num_cols != rhs.num_cols {
            return Err(Error::new(&format!(
                "Incompatible dimensions ({}x{} vs. {}x{})",
                self.num_rows, self.num_cols,
                rhs.num_rows, rhs.num_cols,
            )));
        }

        let added_data: Vec<T> = self
            .data.iter().cloned()
            .zip(rhs.data.iter().cloned())
            .map(|(lhs, rhs)| lhs.add(rhs))
            .collect();
        
        Ok(Self {
            num_rows: self.num_rows,
            num_cols: self.num_cols,
            data: added_data
        })
    }
}

impl<T: Matrixable + ToString> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self
            .rows()
            .map(|row| row
                .iter()
                .map(T::to_string)
                .collect::<Vec<String>>()
                .as_slice()
                .join("\t")
            )
            .map(|string| write!(f, "{}\n", string) )
            .collect::<Result<_, _>>()
    }
}

impl<T: Matrixable + FromStr> FromStr for Matrix<T>
where <T as FromStr>::Err: 'static + ::std::error::Error {
    type Err = Box<::std::error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let data = string
            .lines()
            .map(|line| line
                .split(char::is_whitespace)
                .filter(|word| !word.is_empty())
                .map(str::parse::<T>)
            )
            .map(Iterator::collect::<Result<_, _>>)
            .collect::<Result<_, _>>()
            .map_err(Self::Err::from)?;
        
        Self::from_2d(data).map_err(Self::Err::from)
    }
}
