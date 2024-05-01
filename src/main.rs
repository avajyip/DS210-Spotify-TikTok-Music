use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
mod song;
use song::*;

fn read_tiktok_file(path: &str) -> Vec<Song> {
    let mut result: Vec<Song> = Vec::new();
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
            
            result.push(Song::create_song(&name, &danceability, &energy, &loudness, &valence, &tempo));
        }
    }
    return result;
}

fn read_spotify_file(path: &str) -> Vec<Song> {
    let mut result: Vec<Song> = Vec::new();
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
            
            result.push(Song::create_song(&name, &danceability, &energy, &loudness, &valence, &tempo));
        }
    }
    return result;
}

fn create_attribute_nodes(songs: &Vec<Song>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for song in songs {
        let mut coordinates: Vec<f64> = Vec::new();
        coordinates.push(song.danceability);
        coordinates.push(song.energy);
        coordinates.push(song.loudness);
        coordinates.push(song.valence);
        coordinates.push(song.tempo);
        result.push(coordinates);
    }
    return result;
}

fn get_tracklist(songs: &Vec<Song>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for song in songs {
        result.push(song.title.clone());
    }
    return result;
}

fn get_song_title(songs: &Vec<Song>, attributes: &Vec<f64>) -> String {
    for song in songs {
        if song.danceability == attributes[0] {
            if song.energy == attributes[1] {
                if song.loudness == attributes[2] {
                    if song.valence == attributes[3] {
                        if song.tempo == attributes[4] {
                            return song.title.clone();
                        }
                    }
                }
            }
        }
    }
    return "NA".to_string();
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

fn max_distance(pts: &Vec<Vec<f64>>) -> (f64, Vec<f64>, Vec<f64>) {
    let mut max = 0.0;
    let mut max_pt1: Vec<f64> = Vec::new();
    let mut max_pt2: Vec<f64> = Vec::new();
    let n = pts.len();
    for i in 0..n {
        for j in (i+1)..n {
            let dist = distance(pts[i].to_vec(), pts[j].to_vec());
            if dist > max {
                max = dist;
                max_pt1 = pts[i].to_vec();
                max_pt2 = pts[j].to_vec();
            }
        }
    }
    return (max, max_pt1, max_pt2);
}

fn main() {
    let tiktok2019 = read_tiktok_file("TikTok_songs_2019.tsv");
    //let tiktok2020 = read_tiktok_file("TikTok_songs_2020.tsv");
    //let tiktok2021 = read_tiktok_file("TikTok_songs_2021.tsv");
    //let tiktok2022 = read_tiktok_file("TikTok_songs_2022.tsv");
    //let spotify = read_spotify_file("Spotify_top_charts.tsv");

    let nodes = create_attribute_nodes(&tiktok2019);
    let _tracklist = get_tracklist(&tiktok2019);
    let sample = select_random_sample(&nodes);
    let avg = average_distance(&sample);
    let max = max_distance(&nodes);
    let song1 = get_song_title(&tiktok2019, &max.1);

    println!("{:?}", max);
    println!("{:?}", song1); 
}
