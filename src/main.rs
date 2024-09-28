mod matrix;
mod network;
use matrix::Matrix;
use network::{Network, Activation};
use std::f64::consts::E;

pub const SIGMOID: Activation = Activation {
    function: |x| 1f64 / (1f64 + E.powf(-x)),
    derivative: |x| x * (1.0 - x)
};

fn main() {
    let inputs: Vec<Vec<f64>> = vec![
        vec![0f64, 0f64],
        vec![1f64, 0f64],
        vec![0f64, 1f64],
        vec![1f64, 1f64]
    ];

    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let mut network: Network = Network::new(vec![2, 3, 1], SIGMOID, 0.5f64);

    network.train(inputs, targets, 100000);
    network.feed_forward(Matrix::from(vec![0.0, 0.0])).pretty_print();
    network.feed_forward(Matrix::from(vec![0.0, 1.0])).pretty_print();
    network.feed_forward(Matrix::from(vec![1.0, 0.0])).pretty_print();
    network.feed_forward(Matrix::from(vec![1.0, 1.0])).pretty_print();
}
