use std::error::Error;

use crate::utils::io;

pub fn day2p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d2p1")?;
    let mut total: u64 = 0;

    let str_pairs = file.trim_end().split(",");

    for pair in str_pairs {
        let splits: Vec<&str> = pair.split("-").collect();
        let low: u64 = splits[0].parse()?;
        let high: u64 = splits[1].parse()?;
        for i in low..=high {
            let i_str = i.to_string();
            if !i_str.len().is_multiple_of(2) {
                continue;
            }
            let mid = i_str.len() / 2;
            let (left, right) = i_str.split_at(mid);
            if left == right {
                total += i;
            }
        }
    }

    println!("{total}");
    Ok(())
}

pub fn day2p2() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d2p1")?;
    let mut total: u64 = 0;

    let str_pairs = file.trim_end().split(",");
    let mut v : Vec<u64> = Vec::new();

    for pair in str_pairs {
        let splits: Vec<&str> = pair.split("-").collect();
        let low: u64 = splits[0].parse()?;
        let high: u64 = splits[1].parse()?;
        for i in low..=high {
            let i_str = i.to_string();
            let mlpt = i_str.len().is_multiple_of(2);
            for j in 1..=i_str.len() / 2 {
                if mlpt == j.is_multiple_of(2) {
                    let chunk: Vec<String> = i_str
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(j)
                        .map(|c| c.iter().collect())
                        .collect();

                    if chunk.iter().all(|x| *x == chunk[0]) {
                        total += i;
                        v.push(i);
                        break
                    }
                } else if j == 1 || j == 3 || j == 5 {
                    let chunk: Vec<String> = i_str
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(j)
                        .map(|c| c.iter().collect())
                        .collect();

                    if chunk.iter().all(|x| *x == chunk[0]) {
                        total += i;
                        v.push(i);
                        break
                    }
                }
            }
        }
    }

    println!("{total}");
    Ok(())
}
