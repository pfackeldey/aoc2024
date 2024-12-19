extern crate regex;
use regex::Regex;
use std::fs;

fn solution_a() {
    let content = fs::read_to_string("crates/03/src/data.txt").expect("Unable to read file");

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
    let content = fs::read_to_string("crates/03/src/data.txt").expect("Unable to read file");

    let mut result = 0;
    for chunk in content.split("do()") {
        let doblock = chunk
            .split_once("don't()")
            .map_or(chunk, |(doblock, _)| doblock);
        result += multiply_re_match(doblock);
    }
    println!("Solution b: {:?}", result);
}

fn main() {
    solution_a();
    solution_b();
}
