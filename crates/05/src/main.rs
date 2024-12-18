use std::cmp::Ordering;
use std::fs;

fn solution_a() {
    let contents = fs::read_to_string("data.txt").unwrap();

    let rules = contents
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let mut parts = line.split("|");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            (left, right)
        })
        .collect::<Vec<_>>();

    let mut pages = contents
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| line.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    for page in pages.iter_mut() {
        let unsorted_page = page.clone();
        page.sort_by(|a: &&str, b: &&str| -> Ordering {
            match (rules.contains(&(&a, &b)), rules.contains(&(&b, &a))) {
                (true, _) => Ordering::Less,
                (_, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });
        // I feel like there is a better way to do this
        let matching = page
            .iter()
            .zip(unsorted_page.iter())
            .filter(|&(a, b)| a == b)
            .count();
        if matching == page.len() {
            result += page[(page.len() - 1) / 2].parse::<i32>().unwrap();
        }
    }

    println!("Solution a: {:?}", result);
}

fn solution_b() {
    let contents = fs::read_to_string("data.txt").unwrap();

    let rules = contents
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let mut parts = line.split("|");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            (left, right)
        })
        .collect::<Vec<_>>();

    let mut pages = contents
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| line.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    for page in pages.iter_mut() {
        let unsorted_page = page.clone();
        page.sort_by(|a: &&str, b: &&str| -> Ordering {
            match (rules.contains(&(&a, &b)), rules.contains(&(&b, &a))) {
                (true, _) => Ordering::Less,
                (_, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });
        // I feel like there is a better way to do this
        let matching = page
            .iter()
            .zip(unsorted_page.iter())
            .filter(|&(a, b)| a == b)
            .count();
        if matching != page.len() {
            result += page[(page.len() - 1) / 2].parse::<i32>().unwrap();
        }
    }

    println!("Solution b: {:?}", result);
}

fn main() {
    solution_a();
    solution_b();
}
