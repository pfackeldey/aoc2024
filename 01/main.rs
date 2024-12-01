use std::collections::HashMap;
use std::fs;

fn solution_a() {
    let contents = fs::read_to_string("data.txt").unwrap();

    // read in the file and parse the numbers into two vectors
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for line in contents.lines() {
        let mut nums = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (nums.next(), nums.next()) {
            vec1.push(num1.parse::<i32>().unwrap());
            vec2.push(num2.parse::<i32>().unwrap());
        }
    }

    // sort the vectors in ascending order
    vec1.sort();
    vec2.sort();

    // check that the vectors are the same length
    assert_eq!(vec1.len(), vec2.len());

    // calculate the sum of the absolute differences between the two vectors
    let mut result = 0;
    for i in 0..vec1.len() {
        result += (vec1[i] - vec2[i]).abs();
    }

    println!("Solution a: {:?}", result);
}

fn solution_b() {
    let contents = fs::read_to_string("data.txt").unwrap();

    // read in the file and parse the numbers into two vectors
    // also store the right number occurences in a hashmap
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut count = HashMap::new();
    for line in contents.lines() {
        let mut nums = line.split_whitespace();
        if let (Some(l), Some(r)) = (nums.next(), nums.next()) {
            left.push(l.parse::<i32>().unwrap());

            // store the right number occurences in a hashmap too
            let n = r.parse::<i32>().unwrap();
            right.push(n);
            *count.entry(n).or_insert(0) += 1;
        }
    }

    // sum the product of the left numbers and their occurences in the right numbers
    let mut result = 0;
    for l in left {
        result += l * count.get(&l).unwrap_or(&0);
    }

    println!("Solution b: {:?}", result);
}

fn main() {
    solution_a();
    solution_b();
}
