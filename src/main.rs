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

    let nodes19 = create_attribute_nodes(&tiktok2019);
    let sample19 = select_random_sample(&nodes19, &50);
    let avg19 = average_distance(&sample19);
    let (max_dist19, i19, j19) = max_distance(&nodes19);
    println!("2019 TIKTOK
            \navg distance: {}
            \nmax distance: {}
            \nsongs: {}; {}", 
            avg19, max_dist19, get_song_title(&tiktok2019, &i19), get_song_title(&tiktok2019, &j19));

    let nodes20 = create_attribute_nodes(&tiktok2020);
    let sample20 = select_random_sample(&nodes20, &50);
    let avg20 = average_distance(&sample20);
    let (max_dist20, i20, j20) = max_distance(&nodes20);
    println!("\n2020 TIKTOK
            \navg distance: {}
            \nmax distance: {}
            \nsongs: {}; {}",
            avg20, max_dist20, get_song_title(&tiktok2020, &i20), get_song_title(&tiktok2020, &j20));

    let nodes21 = create_attribute_nodes(&tiktok2021);
    let sample21 = select_random_sample(&nodes21, &50);
    let avg21 = average_distance(&sample21);
    let (max_dist21, i21, j21) = max_distance(&nodes21);
    println!("\n2021 TIKTOK
            \navg distance: {}
            \nmax distance: {}
            \nsongs: {}; {}",
            avg21, max_dist21, get_song_title(&tiktok2021, &i21), get_song_title(&tiktok2021, &j21));

    let nodes22 = create_attribute_nodes(&tiktok2022);
    let sample22 = select_random_sample(&nodes22, &50);
    let avg22 = average_distance(&sample22);
    let (max_dist22, i22, j22) = max_distance(&nodes22);
    println!("\n2022 TIKTOK
            \navg distance: {}
            \nmax distance: {}
            \nsongs: {}; {}",
            avg22, max_dist22, get_song_title(&tiktok2022, &i22), get_song_title(&tiktok2022, &j22));

    let nodes_spotify = create_attribute_nodes(&spotify);
    let sample_spotify = select_random_sample(&nodes_spotify, &200);
    let avg_spotify = average_distance(&sample_spotify);
    let (max_dist_spotify, i_spotify, j_spotify) = max_distance(&nodes_spotify);
    println!("\n2000-2019 SPOTIFY
            \navg distance: {}
            \nmax distance: {}
            \nsongs: {}; {}",
            avg_spotify, max_dist_spotify, get_song_title(&spotify, &i_spotify), get_song_title(&spotify, &j_spotify));

    let tiktok_total_avg = (avg19 + avg20 + avg21 + avg22)/4.0;
    println!("\nTOTAL AVG COMPARISON:
            \ntiktok avg: {}\nspotify avg: {}", tiktok_total_avg, avg_spotify);
}