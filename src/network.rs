use crate::matrix::Matrix;

pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64
}

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64
}

impl Network {
    pub fn new(layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        let mut weights: Vec<Matrix> = Vec::with_capacity(layers.len() - 1);
        let mut biases: Vec<Matrix> = Vec::with_capacity(layers.len() - 1);
        for n in 0..layers.len() - 1 {
            weights[n] = Matrix::random(layers[n + 1], layers[n]);
            biases[n] = Matrix::random(layers[n + 1], 1);
        }
        Network { layers, weights, biases, data: vec![], activation, learning_rate }
    }

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(self.layers[0] == inputs.data.len(), "Invalid Number of Inputs");
        let mut current: Matrix = inputs;
        self.data = vec![current.clone()];
        for n in 0..self.layers.len() - 1 {
            current = &self.weights[n] * &current;
            current += &self.biases[n];
            current.map(self.activation.function);
            self.data.push(current.clone());
        }
        current
    }

    pub fn back_propogate(&mut self, inputs: Matrix, targets: Matrix) {
        let mut errors: Matrix = targets - &inputs;
        let mut gradients: Matrix = inputs.clone().map(self.activation.derivative);

        for n in (0..self.layers.len() -1).rev() {
            gradients = gradients.hadamard_product(&errors).map(|x| x * 0.5);
            self.weights[n] += &gradients * &self.data[n].transpose();
            self.biases[n] += &gradients;
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
        for n in 1..=epochs {
            for input in 0..inputs.len() {
                let output: Matrix = self.feed_forward(Matrix::from(inputs[input].clone()));
                self.back_propogate(output, Matrix::from(targets[input].clone()));
            }
        }
    }
}
