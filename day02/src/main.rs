use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    match part1() {
        Ok(value) => println!("Part1: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    match part2() {
        Ok(value) => println!("Part2: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

fn is_safe(n: &Vec<i32>) -> bool {
    let inc;
    if n[1] > n[0] {
        inc = true;
    } else {
        inc = false;
    }

    for i in 0..n.len() - 1 {
        if inc {
            if n[i+1] <= n[i] || n[i+1] - n[i] > 3 {
                return false;
            } 
        } else {
            if n[i+1] >= n[i] || n[i] - n[i+1] > 3 {
                return false;
            }
        }
    }
    return true;
}


fn part1() -> Result<i32, io::Error> {
    let file = File::open("puzzle")?;
    let reader = io::BufReader::new(file);

    let mut s: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let n: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            if is_safe(&n) {
                s += 1;
            }
        }
    }

    Ok(s)
}


fn part2() -> Result<i32, io::Error> {
    let file = File::open("puzzle")?;
    let reader = io::BufReader::new(file);

    let mut s: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let n: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            if is_safe(&n) {
                s += 1;
                continue;
            } else {

                for i in 0..n.len() {
                    let n2: Vec<i32> = n[0..i].iter().chain(n[i+1..].iter()).copied().collect();
                    if is_safe(&n2) {
                        s += 1;
                        break
                    }
                }
            }
        }
    }

    Ok(s)
}
