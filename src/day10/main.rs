use anyhow::{Result as AResult, bail};
use aoc_2020::parse_input;
use std::collections::HashMap;

fn part1(wall_to_device: &Vec<i32>) -> AResult<i32> {
    let mut counts = HashMap::new();
    for (l, r) in wall_to_device.iter().zip(wall_to_device.iter().skip(1)) {
        let diff = r - l;
        if diff < 1 || diff > 3 {
            bail!("Invalid chained diff {}", diff);
        }
        let count = counts.entry(r - l).or_insert(0);
        *count += 1;
    }
    Ok(counts.get(&1).unwrap_or(&0) * counts.get(&3).unwrap_or(&0))
}

/*
Part 2 Idea:
============
Naively try all combinations, count those with diffs <= 3

Part 2 Idea:
=============
Any pair from pt1 that has a diff of 3 _must_ be used because there are no intermediate combinations.

1. Find subranges between diffs of 3
2. Permute & eval combinations
3. Multiply number of valid combinations from each subrange

Edge case:  What if there are no gaps of 3?
(Practical note:  there are)

Part 2 Idea:
============
There's a limited window of variability; can we somehow iterate between window and adapter and just
solve how many ways there are to get the current joltage via backrefs? (count how many adapters can
feed into this one?).  Then multiply those counts together.

^^ this one ^^

todo: duplicate joltage adapters; does this work?
nb: no dupes in input

 */

fn part2(wall_to_device: &Vec<i32>) -> u64 {
    let mut adapter_path_count : HashMap<i32, u64> = HashMap::new();
    // seed the fact there's "one" possible combination to get to 0 joltage
    adapter_path_count.insert(0, 1);

    //assumes sorted, skips known "wall" path count
    for adapter in wall_to_device.iter().skip(1) {
        let mut incoming_counts = 0;
        for candidate in (adapter - 3) .. *adapter {
            let candidate_paths = adapter_path_count.get(&candidate).unwrap_or(&0);
            incoming_counts +=  candidate_paths;
        }
        adapter_path_count.insert(*adapter, incoming_counts);
    }

    let device_joltage = wall_to_device.last().unwrap();
    let paths_to_device = adapter_path_count.get(device_joltage).unwrap();
    *paths_to_device
}

fn main() -> AResult<()> {
    let mut adapters : Vec<i32> = parse_input(10, |line| line.parse())?;
    adapters.sort();
    let adapter_count = adapters.len();
    let mut wall_to_device = vec![0];
    let device_joltage = *adapters.last().unwrap() + 3;
    wall_to_device.extend(adapters.iter());
    wall_to_device.push(device_joltage);
    println!("Wall joltage: 0, Device joltage: {}, Adapters: {}", device_joltage, adapter_count);

    println!("Part 1: {}", part1(&wall_to_device)?);

    println!("Part 2: {}", part2(&wall_to_device));

    Ok(())
}