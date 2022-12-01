use std::i64;
use std::str::FromStr;

pub fn challenge_1() {
    let data: Vec<String> = std::fs::read_to_string("./challenge1.txt")
        .unwrap()
        .split("\n")
        .into_iter()
        .map(|i| i.to_string())
        .collect();
    println!("Challenge 1 part 1 - {}", most_calories(data.clone()));
    println!("Challenge 1 part 2 - {}", most_calories_part_2(data));
}

fn most_calories(input: Vec<String>) -> i64 {
    let data: Vec<Vec<i64>> = input.iter().fold(Vec::new(), |mut col, i| {
        let create_new = col.is_empty() || i.is_empty();
        if create_new {
            col.push(vec![]);
            if !i.is_empty() {
                col.last_mut()
                    .unwrap()
                    .push(i64::from_str(i.as_str()).unwrap());
            }
        } else {
            col.last_mut()
                .unwrap()
                .push(i64::from_str(i.as_str()).unwrap());
        }
        col
    });
    let mut calories: Vec<i64> = data.iter().map(|v| v.iter().sum()).collect();
    calories.sort();
    *calories.last().unwrap()
}

fn most_calories_part_2(input: Vec<String>) -> i64 {
    let data: Vec<Vec<i64>> = input.iter().fold(Vec::new(), |mut col, i| {
        let create_new = col.is_empty() || i.is_empty();
        if create_new {
            col.push(vec![]);
            if !i.is_empty() {
                col.last_mut()
                    .unwrap()
                    .push(i64::from_str(i.as_str()).unwrap());
            }
        } else {
            col.last_mut()
                .unwrap()
                .push(i64::from_str(i.as_str()).unwrap());
        }
        col
    });
    let mut calories: Vec<i64> = data.iter().map(|v| v.iter().sum()).collect();
    calories.sort();
    calories.split_at(calories.len() - 3).1.iter().sum()
}
