use std::ops::{Add, AddAssign, Sub, SubAssign, Mul};
use rand::Rng;

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub columns: usize
}

impl Matrix {
    pub fn new(data: Vec<f64>, rows: usize, columns: usize) -> Self {
        assert!(data.len() - 1 != rows * columns, "Incorrect number of items."); 
        Self { data, rows, columns }
    }

    pub fn hadamard_product(self, other: &Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] * other.data[n];
        }
        Matrix::new(new_data, self.rows, other.columns)
    }

    pub fn random(rows: usize, columns: usize) -> Self {
        let mut new_data: Vec<f64> = Vec::with_capacity(rows * columns);
        for n in 0..new_data.len() {
            new_data[n] = rand::thread_rng().gen_range(0.0..1.0);
        }
        Matrix::new(new_data, rows, columns)
    }

    pub fn zeroes(rows: usize, columns: usize) -> Self {
        Matrix::new(vec![0f64; rows * columns], rows, columns)
    }

    pub fn transpose(&self) -> Self {
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for row in 0..self.rows {
            for column in 0..self.columns {
                new_data[column * self.rows + row] = self.data[row * self.columns + column];
            }
        }
        Matrix::new(new_data, self.columns, self.rows)
    }

    pub fn map(&mut self, func: fn(&f64) -> f64) -> Matrix {
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        new_data.extend(self.data.iter().map(|&val| func(&val)));
        Matrix::new(new_data, self.columns, self.rows)
    }

    pub fn pretty_print(&self) {
        for n in 0..self.data.len() {
            if n % self.columns == 0 {
                print!("\n");
            }
            print!("{}", self.data[n]);
        }
    }
}

impl From<Vec<f64>> for Matrix {
    fn from(other: Vec<f64>) -> Matrix {
        let l: usize = other.len();
        Matrix::new(other, l, 1)
    }
}

impl Add for Matrix {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] + other.data[n];
        }
        Matrix::new(new_data, self.columns, other.rows)
    }
}

impl Add<&Matrix> for Matrix {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] + other.data[n];
        }
        Matrix::new(new_data, self.columns, other.rows)
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] + other.data[n];
        }
        Matrix::new(new_data, self.columns, other.rows)
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        for n in 0..self.data.len() {
            self.data[n] += other.data[n];
        }
    }
}

impl AddAssign<&Matrix> for Matrix {
    fn add_assign(&mut self, other: &Self) {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        for n in 0..self.data.len() {
            self.data[n] += other.data[n];
        }
    }
}

impl Sub for Matrix {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] - other.data[n];
        }
        Matrix::new(new_data, self.columns, other.rows)
    }
}

impl Sub<&Matrix> for Matrix {
    type Output = Self;
    fn sub(self, other: &Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] - other.data[n];
        }
        Matrix::new(new_data, self.columns, other.rows)
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        for n in 0..self.data.len() {
            self.data[n] -= other.data[n];
        }
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        assert!(self.columns == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.rows * other.columns);
        let mut new_value: f64 = 0f64;
        for row in 0..self.rows {
            for column in 0..other.columns {
                new_value = 0f64;
                for idx in 0..self.columns {
                    new_value += self.data[row * self.columns + idx] * other.data[idx * other.columns + column];
                }
                new_data.push(new_value);
            }
        }
        Matrix::new(new_data, self.rows, other.columns)
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        assert!(self.columns == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.rows * other.columns);
        let mut new_value: f64 = 0f64;
        for row in 0..self.rows {
            for column in 0..other.columns {
                new_value = 0f64;
                for idx in 0..self.columns {
                    new_value += self.data[row * self.columns + idx] * other.data[idx * other.columns + column];
                }
                new_data.push(new_value);
            }
        }
        Matrix::new(new_data, self.rows, other.columns)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.rows * other.columns);
        let mut new_value: f64 = 0f64;
        for row in 0..self.rows {
            for column in 0..other.columns {
                new_value = 0f64;
                for idx in 0..self.columns {
                    new_value += self.data[row * self.columns + idx] * other.data[idx * other.columns + column];
                }
                new_data.push(new_value);
            }
        }
        Matrix::new(new_data, self.rows, other.columns)
    }
}
