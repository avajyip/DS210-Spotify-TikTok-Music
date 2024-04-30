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

fn select_random_sample(pts: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = pts.len();
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
        result.push(pts[i].clone());
    }
    return result;
}

fn distance(node1: Vec<f64>, node2: Vec<f64>) -> f64 {
    let mut squared_distance: f64 = 0.0;
    let n = node1.len();
    for i in 0..n {
        let x = node1[i] - node2[i];
        squared_distance += x.powf(2.0);
    }
    return squared_distance.sqrt();
}

fn average_distance(pts: &Vec<Vec<f64>>) -> f64 {
    let mut sum = 0.0;
    let n = pts.len();
    let num_pairings = (n * (n-1)) / 2;
    for i in 0..n {
        for j in (i+1)..n {
            sum += distance(pts[i].to_vec(), pts[j].to_vec());
        }
    }
    return sum/(num_pairings as f64);
}

fn main() {
    let data = read_file("TikTok_songs_2019.tsv");
    let nodes = create_nodes(&data);
    let _tracklist = tracklist(&data);
    let sample = select_random_sample(&nodes);
    let avg = average_distance(&sample);

    /*println!("ALL:");
    for node1 in &nodes {
        for node2 in &nodes {
            if node1 == node2 {
                continue;
            } else {
                let dist = distance((node1).to_vec(), (node2).to_vec());
                println!("{}", dist);
            }
        }
    }

    println!("SAMPLE:");
    for node1 in &sample {
        for node2 in &sample {
            if node1 == node2 {
                continue;
            } else {
                let dist = distance((node1).to_vec(), (node2).to_vec());
                println!("{}", dist);
            }
        }
    }*/
    println!("{}", avg);
}
