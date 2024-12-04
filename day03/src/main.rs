use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    println!("{}", part1().unwrap());
    println!("{}", part2().unwrap());
}



fn part1() -> Result<i32, io::Error> {
    let file = File::open("puzzle")?;
    let reader = io::BufReader::new(file);

    let mut n = 0;

    let mut s = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            s.push_str(&line);
        }
    }

    for i in 0..s.len() {


        if i+4 < s.len() && &s[i..i+4] == "mul(" {
            let mut lhs = String::new();
            let mut rhs = String::new();
            let mut j = i + 4;
            let mut comma = false;
            let mut valid = false;

            loop {
                let c = s.as_bytes()[j] as char;
                if comma {
                    if c == ')' {
                        valid = true;
                        break;
                    } else if c.is_ascii_digit() {
                        rhs.push(c);
                    } else {
                        break;
                    }
                }

                if !comma {
                    if c == ',' {
                        comma = true;
                    } else if c.is_ascii_digit() {
                        lhs.push(c);
                    } else {
                        break;
                    }
                }

                j += 1;
            }

            if valid {
                let num_lhs: i32 = lhs.parse().unwrap();
                let num_rhs: i32 = rhs.parse().unwrap();
                n += num_lhs * num_rhs
            }
        }
    }


    Ok(n)
}

fn part2() -> Result<i32, io::Error> {
    let file = File::open("puzzle")?;
    let reader = io::BufReader::new(file);

    let mut n = 0;

    let mut s = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            s.push_str(&line);
        }
    }

    let mut mode = true;
    for i in 0..s.len() {
        if i+4 < s.len() && &s[i..i+4] == "do()" {
            mode = true;
            continue;
        }
        if i+7 < s.len() && &s[i..i+7] == "don't()" {
            mode = false;
            continue;
        }


        if i+4 < s.len() && &s[i..i+4] == "mul(" {
            let mut lhs = String::new();
            let mut rhs = String::new();
            let mut j = i + 4;
            let mut comma = false;
            let mut valid = false;

            loop {
                let c = s.chars().nth(j).unwrap();
                if comma {
                    if c == ')' {
                        valid = true;
                        break;
                    } else if c.is_ascii_digit() {
                        rhs.push(c);
                    } else {
                        break;
                    }
                }

                if !comma {
                    if c == ',' {
                        comma = true;
                    } else if c.is_ascii_digit() {
                        lhs.push(c);
                    } else {
                        break;
                    }
                }

                j += 1;
            }

            if valid && mode {
                let num_lhs: i32 = lhs.parse().unwrap();
                let num_rhs: i32 = rhs.parse().unwrap();
                n += num_lhs * num_rhs
            }
        }
    }


    Ok(n)
}
