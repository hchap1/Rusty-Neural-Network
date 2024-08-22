use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub columns: usize
}

impl Matrix {
    pub fn new(data: Vec<f64>, columns: usize, rows: usize) -> Self {
        assert!(data.len() == columns * rows, "Incorrect number of items."); 
        Self { data, rows, columns }
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
        Matrix::new(new_data, columns, rows)
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

impl Sub for Matrix {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] - other.data[n];
        }
        Matrix::new(new_data, columns, rows)
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
        assert!(self.rows == other.columns, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.columns * other.rows);
        for row in 0..self.rows {
            for column in 0..other.columns {
                
            }
        }
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

impl Sub for Matrix {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        assert!(self.columns == other.columns && self.rows == other.rows, "Incompatible Dimensions");
        let mut new_data: Vec<f64> = Vec::with_capacity(self.data.len());
        for n in 0..self.data.len() {
            new_data[n] = self.data[n] - other.data[n];
        }
        Matrix::new(new_data, columns, rows)
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
