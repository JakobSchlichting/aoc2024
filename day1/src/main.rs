use core::panic;
use std::{fs::File, io::{BufReader, Lines}, path::Path};

fn main() {
    let input = utils::file::get_line_itter(Path::new("/Users/jakobschlichting/source/aoc2024/day1/input.txt")).unwrap();
    let res1 = one(input);
    let input = utils::file::get_line_itter(Path::new("/Users/jakobschlichting/source/aoc2024/day1/input.txt")).unwrap();
    let res2 = two(input);
    println!("Result 1: {:?}", res1.unwrap());
    println!("Result 2: {:?}", res2);
}

fn two(input: Lines<BufReader<File>>) -> u64 {
    let (left, right) = get_vecs(input);
    left
        .iter()
        .map(|left_num| {
            *left_num * right.iter().filter(|right_num| {
                *left_num == *(*right_num)
            }).count() as u64
        }).sum()
}

fn one(input: Lines<BufReader<File>>) -> Option<u64> {
    let (mut left_numbers, mut right_nubers) = get_vecs(input);
    left_numbers.sort();
    right_nubers.sort();
    left_numbers
        .iter()
        .zip(right_nubers.iter())
        .map(|(left, right)| { right.abs_diff(*left) })
        .reduce(|acc, b| { acc + b})
}

fn get_vecs(lines: Lines<BufReader<File>>) -> (Vec<u64>, Vec<u64>) {
    lines
        .filter_map(|line| {
            match line {
                Err(_) => None,
                Ok(l) => Some(l),
            }
        }).map(|e| split_line(e))
        .fold((Vec::new(), Vec::new()), |(mut left_acc, mut right_acc), (left, right)| {
            left_acc.push(left);
            right_acc.push(right);
            (left_acc, right_acc)
        })
}

fn split_line(line: String) -> (u64, u64) {
    let mut splits: Vec<&str> = line.split_whitespace().collect();
    if splits.len() != 2 {
        panic!("line without two numbers. len is {}", splits.len())
    }

    (splits.pop().unwrap().parse::<u64>().unwrap(), splits.pop().unwrap().parse::<u64>().unwrap())
}
