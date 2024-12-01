use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let columns: Vec<&str> = line.split_whitespace().collect();

            if let Some(first) = columns.get(0) {
                col1.push(first.parse::<i32>().unwrap());  // Parse into i32
            }

            if let Some(second) = columns.get(1) {
                col2.push(second.parse::<i32>().unwrap());  // Parse into i32
            }

        }
    }
    let mut similarity_score: Vec<i32> = Vec::new();
    for element in col1 {
        let occur = count_occurrence(col2.clone(), element);
        similarity_score.push(occur*element);
    }
    let sum: i32 = similarity_score.clone().into_iter().sum();
    println!("Anwser: {:?}", sum);

    // Part 1///
    // col1.sort();
    // col2.sort();
    // let diff: Vec<i32> = elementwise_subtraction(col2.clone(), col1.clone());
    // let sum: i32 = diff.clone().into_iter().sum();
    // // println!("Column 1: {:?}", col1);
    // // println!("Column 2: {:?}", col2);
    // println!("Anwser: {:?}", sum);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn elementwise_subtraction(vec_a: Vec<i32>, vec_b: Vec<i32>) -> Vec<i32> {
    vec_a.into_iter()
        .zip(vec_b)
        .map(|(a, b)| (a - b).abs())
        .collect()
}

pub fn count_occurrence(vec: Vec<i32>, target: i32) -> i32 {
    vec.iter().filter(|&&x| x == target).count() as i32
}
