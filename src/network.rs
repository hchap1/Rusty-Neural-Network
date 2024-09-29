use crate::matrix::Matrix;
use std::io::{self, Write};
use crate::activations::{Activation, SIGMOID};
use std::fs::{write, read_to_string};

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
}

impl Network {
    pub fn new(layers: Vec<usize>, activation: Activation) -> Self {
        let mut weights: Vec<Matrix> = vec![];
        let mut biases: Vec<Matrix> = vec![];
        for n in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[n + 1], layers[n]));
            biases.push(Matrix::random(layers[n + 1], 1));
        }
        Network { layers, weights, biases, data: vec![], activation }
    }

    pub fn save(&self, filename: String) {
        let serialised = self.serialise();
        let _ = write(filename, serialised);
    }

    pub fn load(filename: String) -> Result<Self, String> {
        match read_to_string(filename) {
            Ok(raw) => {
                let content: Vec<String> = raw.lines().map(|x| x.to_string()).collect();
                assert!(content.len() == 3, "Invalid file.");
                let layers: Vec<usize> = content[0].split(", ").map(|x| if let Ok(n) = x.parse::<usize>() {n} else {
                    panic!("Found invalid layer, expected usize.");
                }).collect::<Vec<usize>>();
                let serial_weights: Vec<String> = content[1].split(" | ").map(|x| x.to_string()).collect();
                let serial_biases: Vec<String> = content[2].split(" | ").map(|x| x.to_string()).collect();
                let weights: Vec<Matrix> = serial_weights.iter().map(|x| Matrix::from(x)).collect::<Vec<Matrix>>();
                let biases: Vec<Matrix> = serial_biases.iter().map(|x| Matrix::from(x)).collect::<Vec<Matrix>>();
                Ok(Network {
                    layers,
                    weights,
                    biases,
                    data: vec![],
                    activation: SIGMOID
                })
            }
            Err(_) => {
                Err(String::from("File operation failed. Check path."))
            }
        }
    }

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(self.layers[0] == inputs.data.len(), "Invalid Number of Inputs");
        let mut current: Matrix = inputs;
        self.data = vec![current.clone()];
        for n in 0..self.layers.len() - 1 {
            current = &self.weights[n] * &current;
            current = &current + &self.biases[n];
            current = current.map(self.activation.function);
            self.data.push(current.clone());
        }
        current
    }

    pub fn predict(&mut self, input: Vec<f64>) -> Vec<f64> {
        self.feed_forward(Matrix::from(input)).data
    }

    pub fn back_propogate(&mut self, inputs: Matrix, targets: Matrix) {
        let mut errors: Matrix = targets - &inputs;
        let mut gradients: Matrix = inputs.clone().map(self.activation.derivative);

        for n in (0..self.layers.len() -1).rev() {
            gradients = gradients.hadamard_product(&errors).map(|x| x * 0.5);
            self.weights[n] += &(&gradients * &self.data[n].transpose());
            self.biases[n] += &gradients;
            errors = self.weights[n].transpose() * &errors;
            gradients = self.data[n].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
        for e in 1..=epochs {
            print!("{e}/{epochs}\r");
            let _ = io::stdout().flush();
            for input in 0..inputs.len() {
                let output: Matrix = self.feed_forward(Matrix::from(inputs[input].clone()));
                self.back_propogate(output, Matrix::from(targets[input].clone()));
            }
        }
    }

    pub fn serialise(&self) -> String {
        let serial_layers: String = self.layers.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        let serial_weights: Vec<String> = self.weights.iter().map(|x| x.serialise()).collect::<Vec<String>>();
        let serial_biases: Vec<String> = self.biases.iter().map(|x| x.serialise()).collect::<Vec<String>>();
        format!("{}\n{}\n{}", serial_layers, serial_weights.join(" | "), serial_biases.join(" | "))
    }
}
