use std::fs::File;
use std::io::prelude::*;
mod song;
use song::*;

fn read_tiktok_file(path: &str) -> Vec<(String, f64, f64, f64, f64, f64)> {
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

fn read_spotify_file(path: &str) -> Vec<(String, f64, f64, f64, f64, f64)> {
    let mut result: Vec<(String, f64, f64, f64, f64, f64)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();

    for (i, line) in buf_reader.enumerate() {
        if i == 0 {
            continue;
        } else {
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split('\t').collect();
            let name = v[1].parse::<String>().unwrap();
            let danceability = v[6].parse::<f64>().unwrap();
            let energy = v[7].parse::<f64>().unwrap();
            let loudness = v[9].parse::<f64>().unwrap();
            let valence = v[15].parse::<f64>().unwrap();
            let tempo = v[16].parse::<f64>().unwrap();
            
            result.push((name, danceability, energy, loudness, valence, tempo));
        }
    }
    return result;
}

fn main() {
    let tiktok2019 = read_tiktok_file("TikTok_songs_2019.tsv");
    let tiktok2020 = read_tiktok_file("TikTok_songs_2020.tsv");
    let tiktok2021 = read_tiktok_file("TikTok_songs_2021.tsv");
    let tiktok2022 = read_tiktok_file("TikTok_songs_2022.tsv");
    let spotify = read_spotify_file("Spotify_top_charts.tsv");

    let nodes = create_nodes(&tiktok2019);
    let _tracklist = tracklist(&tiktok2019);
    let sample = select_random_sample(&nodes);
    let avg = average_distance(&sample);

    println!("{}", avg); 
}
