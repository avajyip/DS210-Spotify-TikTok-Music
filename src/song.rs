use rand::Rng;

pub struct Song {
    pub title: String,
    pub danceability: f64,
    pub energy: f64,
    pub speechiness: f64,
    pub valence: f64,
    pub tempo: f64,
}

impl Song {
    pub fn create_song(ti: &String, d: &f64, e: &f64, s: &f64, v: &f64, te: &f64) -> Song {
        return Song{title: ti.clone(), danceability: *d, energy: *e, speechiness: *s, valence: *v, tempo: *te};
    }
}

pub fn create_attribute_nodes(songs: &Vec<Song>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    for song in songs {
        let mut coordinates: Vec<f64> = Vec::new();
        coordinates.push(song.danceability);
        coordinates.push(song.energy);
        coordinates.push(song.speechiness);
        coordinates.push(song.valence);
        coordinates.push(song.tempo);
        result.push(coordinates);
    }
    return result;
}

pub fn get_song_title(songs: &Vec<Song>, attributes: &Vec<f64>) -> String {
    for song in songs {
        if song.danceability == attributes[0] {
            if song.energy == attributes[1] {
                if song.speechiness == attributes[2] {
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

pub fn select_random_sample(pts: &Vec<Vec<f64>>, num_samples: &usize) -> Vec<Vec<f64>> {
    let n = pts.len();
    let mut result: Vec<Vec<f64>> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let mut count = 0;
    while count < *num_samples {
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

pub fn distance(node1: Vec<f64>, node2: Vec<f64>) -> f64 {
    let mut squared_distance: f64 = 0.0;
    let n = node1.len();
    for i in 0..n {
        let x = node1[i] - node2[i];
        squared_distance += x.powf(2.0);
    }
    return squared_distance.sqrt();
}

pub fn average_distance(pts: &Vec<Vec<f64>>) -> f64 {
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

pub fn max_distance(pts: &Vec<Vec<f64>>) -> (f64, Vec<f64>, Vec<f64>) {
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