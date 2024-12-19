use std::fs;

fn strip<'a>(patterns: Vec<&'a str>, design: &'a str) -> &'a str {
    let mut mod_design = design;
    for pattern in &patterns {
        mod_design = mod_design.strip_prefix(pattern).unwrap_or(mod_design);
    }
    // if it is the same, then we didn't strip anything
    if mod_design == design {
        return design;
    // if it is empty, then we stripped everything
    } else if mod_design.len() == 0 {
        return mod_design;
    // try again
    } else {
        return strip(patterns, mod_design);
    }
}

fn solution_a() {
    let contents = fs::read_to_string("crates/19/src/data.txt").unwrap();

    let data: Vec<&str> = contents.lines().collect();
    let patterns: Vec<&str> = data[0].split(", ").collect();
    let designs: Vec<&str> = data[2..].to_vec();

    let mut count = 0;
    for design in &designs {
        let modified_design = strip(patterns.clone(), design);
        // we're only counting the ones that are stripped completely
        if modified_design.len() == 0 {
            println!("{:?}", design);
            println!("{:?}", modified_design);
            count += 1;
        }
    }

    println!("Solution a: {:?}", count);
}

fn solution_b() {
    println!("Solution b: {:?}", 0);
}

fn main() {
    solution_a();
    solution_b();
}
