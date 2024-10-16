mod matrix;
mod network;
mod activations;

use std::env::args;
use matrix::Matrix;
use network::Network;
use activations::SIGMOID;
use std::fs::read_to_string;

fn team_name_to_id(team_name: &str) -> Option<f64> {
    match team_name.to_lowercase().as_str() {
        "cutters" => Some(0.0),
        "devils" => Some(1.0),
        "bears" => Some(2.0),
        "clydesdales" => Some(3.0),
        "falcons" => Some(4.0),
        "magpies" => Some(5.0),
        "wm" => Some(6.0),
        "seagulls" => Some(8.0),
        "tigers" => Some(7.0),
        "capras" => Some(9.0),
        _ => None
    }
}

fn id_to_team_name(id: f64) -> Option<String> {
    match id {
        0.0 => Some("Cutters".to_string()),
        1.0 => Some("Devils".to_string()),
        2.0 => Some("Bears".to_string()),
        3.0 => Some("Clydesdales".to_string()),
        4.0 => Some("Falcons".to_string()),
        5.0 => Some("Magpies".to_string()),
        6.0 => Some("WM Seagulls".to_string()),
        7.0 => Some("Tigers".to_string()),
        8.0 => Some("Seagulls".to_string()),
        9.0 => Some("Capras".to_string()),
        _ => None
    }
}

fn main() {
    let mut args = args();
    let _ = args.next();
    let team_a = team_name_to_id(&args.next().unwrap()).unwrap();
    let team_b = team_name_to_id(&args.next().unwrap()).unwrap();
    let mut network: Network = match Network::load(String::from("psmt_network.hnn")) {
        Ok(network) => network,
        Err(_) => {
            let rows = match read_to_string("psmt_data.txt") {
                Ok(raw) => raw.lines().map(|x| x.split("\t").map(|y| y.parse::<f64>().unwrap()).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>(),
                Err(_) => panic!("Couldn't open psmt_data.txt.")
            };

            let mut inputs: Vec<Vec<f64>> = vec![];
            let mut target: Vec<Vec<f64>> = vec![];

            for (row_idx, row) in rows.iter().enumerate() {
                for (col_idx, item) in row.iter().enumerate() {
                    inputs.push(vec![row_idx as f64, col_idx as f64]);
                    target.push(vec![*item]);
                }
            }

            let mut network: Network = Network::new(vec![2, 3, 1], SIGMOID);
            network.train(inputs, target, 1000);
            network.save(String::from("psmt_network.hnn"));
            network
        }
    };
    network.feed_forward(Matrix::from(vec![team_a, team_b])).pretty_print();
}
