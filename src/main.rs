mod matrix;
mod network;
mod activations;

use std::env::args;
use network::Network;
use activations::SIGMOID;
use std::fs::read_to_string;

fn main() {
    /*
    rusty-neural-network train <DESTINATION FILEPATH> <TRAINING FILEPATH>
    rusty-neural-network predict <MODEL FILEPATH> <DATA>
    */
    let args = args().collect::<Vec<String>>();
    let mut to_predict: Option<Vec<f64>> = None;
    let (mut network, mp) = if args.len() >= 4 {
        let command: String = args[1].clone();
        let model_path: String = args[2].clone();
        let argument: String = args[3].clone();
        match command.as_str() {
            "train" => {
                // Structure of training file should be:
                // 1 input1, input2, ... inputN | output1, output2, ..., outputN
                // 2 input1, input2, ... inputN | output1, output2, ..., outputN
                if let Ok(raw) = read_to_string(argument.clone() + ".td") {
                    let training_cases: Vec<String> = raw.lines().map(|x| x.to_string()).collect::<Vec<String>>();
                    let mut inputs: Vec<Vec<f64>> = vec![];
                    let mut targets: Vec<Vec<f64>> = vec![];
                    for c in training_cases {
                        let case = c.split(" | ").map(|x| x.to_string()).collect::<Vec<String>>();
                        assert!(case.len() == 2, "Improper input data format.");
                        inputs.push(case[0].split(", ").map(|x| match x.parse::<f64>() {
                            Ok(n) => { n }
                            Err(_) => {
                                panic!("NAN Found in training data.");
                            }
                        }).collect());
                        targets.push(case[1].split(", ").map(|x| match x.parse::<f64>() {
                            Ok(n) => { n }
                            Err(_) => {
                                panic!("NAN Found in training data.");
                            }
                        }).collect());
                    }
                    let layers: Vec<usize> = vec![inputs[0].len(), inputs[0].len() * 2, targets[0].len()];
                    println!("{layers:?} <- LAYERS");
                    let mut nw = Network::new(layers, SIGMOID);
                    nw.train(inputs, targets, 10000);
                    (nw, model_path)
                } else {
                    panic!("Could not read from file {argument}");
                }
            }
            "predict" => {
                to_predict = Some(argument.split(", ").map(|x| match x.parse::<f64>() {
                    Ok(n) => { n }
                    Err(_) => { panic!("NAN in data") }
                }).collect());
                match Network::load(model_path.clone()) {
                    Ok(nw) => { (nw, model_path) }
                    Err(_) => { panic!("Could not load network from file"); }
                }
            }
            e => {
                panic!("No such command: {e}");
            }
        }
    } else { panic!("Couldn't load 3 arguments") };
    if let Some(input) = to_predict {
        let prediction = network.predict(input);
        match read_to_string(mp + ".cl") {
            Ok(raw) => {
                let classifications: Vec<String> = raw.lines().map(|x| x.to_string()).collect::<Vec<String>>();
                let classification = classifications.last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
                let result: usize = prediction[0].round() as usize;
                println!("[DONE] -> {}", classification[result]);
            }
            Err(_) => {
                panic!("Could not access classifications");
            }
        }
    } else {
        network.save(mp + ".nw");
    }
}
