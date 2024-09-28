use std::ops::{Add, AddAssign, Sub, Mul, MulAssign};
use rand::Rng;

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub columns: usize
}

impl Matrix {
    pub fn new(data: Vec<f64>, rows: usize, columns: usize) -> Self {
        assert!(data.len() == rows * columns, "Incorrect number of items."); 
        Self { data, rows, columns }
    }

    pub fn hadamard_product(self, other: &Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = vec![];
        for n in 0..self.data.len() {
            new_data.push(self.data[n] * other.data[n]);
        }
        Matrix::new(new_data, self.rows, other.columns)
    }

    pub fn random(rows: usize, columns: usize) -> Self {
        let mut new_data: Vec<f64> = vec![];
        for _ in 0..(rows * columns) {
            new_data.push(rand::thread_rng().gen_range(0.0..1.0));
        }
        Matrix::new(new_data, rows, columns)
    }

    pub fn zeroes(rows: usize, columns: usize) -> Self {
        Matrix::new(vec![0f64; rows * columns], rows, columns)
    }

    pub fn transpose(&self) -> Self {
        let mut new_data: Vec<f64> = vec![0f64; self.rows * self.columns];
        for row in 0..self.rows {
            for column in 0..self.columns {
                new_data[column * self.rows + row] = self.data[row * self.columns + column];
            }
        }
        Matrix::new(new_data, self.columns, self.rows)
    }

    pub fn map(&self, func: fn(&f64) -> f64) -> Matrix {
        let new_data = self.data.iter().map(|&val| func(&val)).collect();
        Matrix::new(new_data, self.rows, self.columns)
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

impl AddAssign<&Matrix> for Matrix {
    fn add_assign(&mut self, other: &Self) {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        for n in 0..(self.rows * self.columns) {
            self.data[n] += other.data[n];
        }
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = vec![];
        for n in 0..self.rows * other.columns {
            new_data.push(self.data[n] + other.data[n]);
        }
        Matrix::new(new_data, self.rows, other.columns)
    }
}

impl Sub<&Matrix> for Matrix {
    type Output = Self;
    fn sub(self, other: &Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions {}, {} x {}, {}",
        self.rows, self.columns, other.rows, other.columns);
        let mut new_data: Vec<f64> = vec![];
        for n in 0..(self.rows * other.columns) {
            new_data.push(self.data[n] - other.data[n]);
        }
        Matrix::new(new_data, self.rows, other.columns)
    }
}

impl MulAssign<&Matrix> for Matrix {
    fn mul_assign(&mut self, other: &Self) {
        assert!(self.columns == other.rows, "Incompatible Dimensions");
        
        let mut new_data: Vec<f64> = vec![0.0; self.rows * other.columns];
        
        for row in 0..self.rows {
            for column in 0..other.columns {
                let mut new_value = 0f64;
                for idx in 0..self.columns {
                    new_value += self.data[row * self.columns + idx] * other.data[idx * other.columns + column];
                }
                new_data[row * other.columns + column] = new_value;
            }
        }

        *self = Matrix::new(new_data, self.rows, other.columns);
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;
    
    fn mul(self, other: &Self) -> Self {
        assert!(self.columns == other.rows, "Incompatible Dimensions");
        
        let mut new_data: Vec<f64> = vec![0f64; self.rows * other.columns];
        
        for row in 0..self.rows {
            for column in 0..other.columns {
                let mut new_value = 0f64;
                for idx in 0..self.columns {
                    new_value += self.data[row * self.columns + idx] * other.data[idx * other.columns + column];
                }
                new_data[row * other.columns + column] = new_value;
            }
        }
        
        Matrix::new(new_data, self.rows, other.columns)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.rows, "Incompatible Dimensions");

        let mut new_data: Vec<f64> = vec![0f64; self.rows * other.columns];
        
        for row in 0..self.rows {
            for column in 0..other.columns {
                let mut new_value = 0f64;
                for idx in 0..self.columns {
                    new_value += self.data[row * self.columns + idx] * other.data[idx * other.columns + column];
                }
                new_data[row * other.columns + column] = new_value;
            }
        }
        
        Matrix::new(new_data, self.rows, other.columns)
    }
}
