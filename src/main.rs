use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn read_file(path: &str) -> Vec<(String, f64, f64, f64, f64, f64)> {
    let mut result: Vec<(String, f64, f64, f64, f64, f64)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    for (i, line) in buf_reader.enumerate() {
        if i == 0 {
            continue;
        } else {
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split('\t').collect();
            let name = v[0].parse::<String>().unwrap();
            let danceability = v[5].parse::<f64>().unwrap();
            let energy = v[6].parse::<f64>().unwrap();
            let loudness = v[7].parse::<f64>().unwrap();
            let valence = v[14].parse::<f64>().unwrap();
            let tempo = v[15].parse::<f64>().unwrap();
            
            result.push((name, danceability, energy, loudness, valence, tempo));
        }
    }
    return result;
}

fn create_nodes(vals: &Vec<(String, f64, f64, f64, f64, f64)>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for val in vals {
        let mut coordinates: Vec<f64> = Vec::new();
        coordinates.push(val.1);
        coordinates.push(val.2);
        coordinates.push(val.3);
        coordinates.push(val.4);
        coordinates.push(val.5);
        result.push(coordinates);
    }
    return result;
}

fn tracklist(vals: &Vec<(String, f64, f64, f64, f64, f64)>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for val in vals {
        result.push(val.0.clone());
    }
    return result;
}

fn select_random_sample(vals: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = vals.len();
    let mut result: Vec<Vec<f64>> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let mut count = 0;
    while count < 50 {
        let x = rand::thread_rng().gen_range(0..n);
        if !indices.contains(&x) {
            indices.push(x);
            count += 1;
        }
    }
    for i in indices {
        result.push(vals[i].clone());
    }
    return result;
}

fn distance(node1: Vec<f64>, node2: Vec<f64>) -> f64 {
    let mut squared_distance: f64 = 0.0;
    for i in 0..5 {
        let x = node1[i] - node2[i];
        squared_distance += x.powf(2.0);
    }
    return squared_distance.sqrt();
}

/*fn average_distance() {

}*/

fn main() {
    let data = read_file("TikTok_songs_2019.tsv");
    let nodes = create_nodes(&data);
    let _tracklist = tracklist(&data);
    let sample = select_random_sample(&nodes);
}
