use rand::Rng;

pub fn create_nodes(vals: &Vec<(String, f64, f64, f64, f64, f64)>) -> Vec<Vec<f64>> {
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

pub fn tracklist(vals: &Vec<(String, f64, f64, f64, f64, f64)>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for val in vals {
        result.push(val.0.clone());
    }
    return result;
}

pub fn select_random_sample(pts: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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