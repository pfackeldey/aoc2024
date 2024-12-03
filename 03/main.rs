extern crate regex;
use regex::Regex;
use std::fs;

fn solution_a() {
    let content = fs::read_to_string("data.txt").expect("Unable to read file");

    let result = multiply_re_match(&content);
    println!("Solution a: {:?}", result);
}

fn multiply_re_match(chunk: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    for cap in re.captures_iter(chunk) {
        let a: i32 = cap[1].parse::<i32>().unwrap();
        let b: i32 = cap[2].parse::<i32>().unwrap();
        result += a * b;
    }
    result
}

fn solution_b() {
    let content = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut result = 0;
    for chunk in content.split("do()") {
        if chunk.contains("don't()") {
            // as soon as we find a don't() block, we can split the chunk
            // and find all the mul() operations within the do() block (the first part)
            if let Some((doblock, _)) = chunk.split_once("don't()") {
                let tmp = multiply_re_match(doblock);
                result += tmp;
            }
        } else {
            // if there is no don't() block, we can just find all the mul() operations
            let tmp = multiply_re_match(chunk);
            result += tmp;
        }
    }
    println!("Solution b: {:?}", result);
}

fn main() {
    solution_a();
    solution_b();
}
