use std::fs::File;
use std::io::prelude::*;
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
            let speechiness = v[10].parse::<f64>().unwrap();
            let valence = v[14].parse::<f64>().unwrap();
            let tempo = (v[15].parse::<f64>().unwrap())/250.0;
            
            result.push(Song::create_song(&name, &danceability, &energy, &speechiness, &valence, &tempo));
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
            let speechiness = v[11].parse::<f64>().unwrap();
            let valence = v[15].parse::<f64>().unwrap();
            let tempo = (v[16].parse::<f64>().unwrap())/250.0;
            
            result.push(Song::create_song(&name, &danceability, &energy, &speechiness, &valence, &tempo));
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

    let files = vec![tiktok2019, tiktok2020, tiktok2021, tiktok2022, spotify];
    let mut averages: Vec<f64> = Vec::new();

    for i in 0..5 {
        let nodes = create_attribute_nodes(&files[i]);
        if i == 4 { //SPOTIFY
            let sample = select_random_sample(&nodes, &200);
            let avg = average_distance(&sample);
            averages.push(avg);
            let (max_dist, song1_attributes, song2_attributes) = max_distance(&nodes);
            println!("\n2000-2019 SPOTIFY
            \navg distance between random sample of 200 songs: {}
            \nmax distance between two songs: {}
            \nmost different songs: {}; {}", 
            avg, max_dist, get_song_title(&files[i], &song1_attributes), get_song_title(&files[i], &song2_attributes));
        } else { //TIKTOK
            let sample = select_random_sample(&nodes, &50);
            let avg = average_distance(&sample);
            averages.push(avg);
            let header: String = (2019+i).to_string();
            let (max_dist, song1_attributes, song2_attributes) = max_distance(&nodes);
            println!("\n{} TIKTOK
            \navg distance between random sample of 50 songs: {}
            \nmax distance between two songs: {}
            \nmost different songs: {}; {}", 
            header, avg, max_dist, get_song_title(&files[i], &song1_attributes), get_song_title(&files[i], &song2_attributes));
        }
    }

    let tiktok_total_avg = (averages[0] + averages[1] + averages[2] + averages[3]) / 4.0;
    
    let spotify_avg = averages[4];
    println!("\nAVERAGE DISTANCE COMPARISON
            \ntiktok avg distance: {} \nspotify avg distance: {}", tiktok_total_avg, spotify_avg);

    if tiktok_total_avg < spotify_avg {
        println!("TikTok viral songs are, on average, more similar than Spotify Top Charts songs.")
    } else {
        println!("TikTok viral songs are, on average, less similar than Spotify Top Charts songs.")
    }
}