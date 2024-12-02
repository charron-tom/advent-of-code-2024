use std::fs;
use std::collections::HashMap;


fn main() {
    let p1 = part1();
    let p2 = part2();

    println!("{}", p1);
    println!("{}", p2);
}


fn part1() -> i32 {
    let contents = fs::read_to_string("puzzle")
        .expect("Could not find file.");

    let mut v: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    let mut s: i32 = 0;

    for line in contents.lines() {
        let parts: [&str; 2] = line.split("   ").collect::<Vec<_>>().try_into().unwrap();
        let n: i32 = parts[0].parse().unwrap();
        let n2: i32 = parts[1].parse().unwrap();

        v.push(n);
        v2.push(n2);
    }

    v.sort();
    v2.sort();

    for i in 0..v.len() {
        s += (v[i] - v2[i]).abs();
    }

    return s;
}


fn part2() -> i32 {
    let contents = fs::read_to_string("puzzle")
        .expect("Could not find file.");

    let mut v: Vec<i32> = Vec::new();
    let mut m2 = HashMap::new();
    let mut s: i32 = 0;

    for line in contents.lines() {
        let parts: [&str; 2] = line.split("   ").collect::<Vec<_>>().try_into().unwrap();
        let n: i32 = parts[0].parse().unwrap();
        let n2: i32 = parts[1].parse().unwrap();

        v.push(n);
        let count = m2.entry(n2).or_insert(0);
        *count += 1;
    }

    v.sort();

    for (i, &item) in v.iter().enumerate() {
        s += m2.get(&item).copied().unwrap_or(0) * v[i];
    }

    return s;
}
