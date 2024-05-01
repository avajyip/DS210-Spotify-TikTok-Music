pub struct Song {
    pub title: String,
    pub danceability: f64,
    pub energy: f64,
    pub loudness: f64,
    pub valence: f64,
    pub tempo: f64,
}

impl Song {
    pub fn create_song(ti: &String, d: &f64, e: &f64, l: &f64, v: &f64, te: &f64) -> Song {
        return Song{title: ti.clone(), danceability: *d, energy: *e, loudness: *l, valence: *v, tempo: *te};
    }
}