use std::fs;

fn solution_a() {
    let contents = fs::read_to_string("data.txt").unwrap();

    let mut n_safe = 0;
    for line in contents.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        n_safe += check(nums) as i32;
    }
    println!("Solution a: {:?}", n_safe);
}

fn check(nums: Vec<i32>) -> bool {
    // calculate differences between each number
    let diffs: Vec<i32> = nums.windows(2).map(|w| w[0] - w[1]).collect();
    // check if:
    // 1. all absolute differences are non-zero and less than or equal to 3
    // 2. all differences are either positive or negative
    return diffs.iter().all(|&v| v != 0 && v.abs() <= 3)
        && (diffs.iter().all(|&v| v > 0) || diffs.iter().all(|&v| v < 0));
}

fn solution_b() {
    let contents = fs::read_to_string("data.txt").unwrap();

    let mut n_safe = 0;
    for line in contents.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        for i in 0..nums.len() {
            let mut temp_nums = nums.clone();
            temp_nums.remove(i);
            if check(temp_nums) {
                n_safe += 1;
                break;
            }
        }
    }

    println!("Solution b: {:?}", n_safe);
}

fn main() {
    solution_a();
    solution_b();
}
