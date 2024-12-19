use std::fs;

fn matches<'a>(patterns: Vec<&'a str>, design: &'a str) -> usize {
    let mut mod_design = design;
    for pattern in &patterns {
        mod_design = mod_design.strip_prefix(pattern).unwrap_or(mod_design);
    }
    // if it is the same, then we didn't strip anything
    // so we can return 0 as another round will not change anything
    if mod_design == design {
        return 0;
    // if it is empty, then we stripped everything
    // success!
    } else if mod_design.len() == 0 {
        return 1;
    // try again with the stripped design
    } else {
        return matches(patterns, mod_design);
    }
}

fn solution_a() {
    let contents = fs::read_to_string("crates/19/src/data.txt").unwrap();

    let data: Vec<&str> = contents.lines().collect();
    let patterns: Vec<&str> = data[0].split(", ").collect();
    let designs: Vec<&str> = data[2..].to_vec();

    let mut nmatches = 0;
    for design in &designs {
        nmatches += matches(patterns.clone(), design);
    }

    println!("Solution a: {:?}", nmatches);
}

fn solution_b() {
    println!("Solution b: {:?}", 0);
}

fn main() {
    solution_a();
    solution_b();
}
