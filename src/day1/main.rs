use aoc_2020::parse_input;
use std::io::{BufReader, BufRead, Error};
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let expense_items : Result<Vec<i32>, _> = parse_input(1, |line| line.parse());
    let expense_items = expense_items?;

    let pairs = expense_items.iter().combinations(2);

    for pair in pairs {
        // println!("{:?}", pair);
        match pair[..2] {
            [l, r] if l + r == 2020 => println!("found {} + {};  mult = {}", l, r, l * r),
            _ => ()
        }
    }

    let triplets = expense_items.iter().combinations(3);
    for triplet in triplets {
        match triplet[..3] {
            [x, y, z] if x + y + z == 2020 => println!("found {} + {} + {}; mult = {}", x, y, z, x * y * z),
            _ => ()
        }
    }

    Ok(())
}
