use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut safe_vec: Vec<bool> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let row: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            if is_safe(&row){
                safe_vec.push(true);
            }
            // println!("{:?}", is_safe(&row));
            // println!("{:?}", check_levels(&row));
            // // println!("{:?}", is_sorted(&row));
        }
    }
    println!("{:?}", safe_vec.len());
}

fn is_safe(vec: &Vec<i32>) -> bool {
    is_sorted(&vec) && check_levels(&vec)
}

fn check_levels(vec: &Vec<i32>) -> bool {
    // println!("{:?}", vec.windows(2));
    vec.windows(2).all(|w| ((w[0] - w[1]).abs()) <= 3)
}

fn is_sorted(vec: &Vec<i32>) -> bool {
    let dec = vec.windows(2).all(|w| w[0] > w[1]);
    let inc = vec.windows(2).all(|w| w[0] < w[1]);
    dec || inc
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
