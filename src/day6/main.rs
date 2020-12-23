use aoc_2020::{read_input};
use anyhow::{bail, Result};
use std::collections::HashSet;

struct GroupAnswers {
    individual_answers: Vec<HashSet<char>>
}

impl GroupAnswers {
    fn parse(input: &str) -> Result<GroupAnswers> {
        let question_keys = 'a' ..= 'z';
        let mut individual_answers = Vec::new();
        for line in input.lines() {
            let mut answers = HashSet::new();
            for c in line.chars() {
                if !question_keys.contains(&c) {
                    bail!("Invalid answer set: {}", line);
                }
                answers.insert(c);
            }
            individual_answers.push(answers);
        }
        Ok(GroupAnswers { individual_answers })
    }

    fn distinct_any_count(&self) -> usize {
        let mut union : HashSet<char> = HashSet::new();
        for answers in self.individual_answers.iter() {
            union.extend(answers);
        }
        union.len()
    }

    fn distinct_all_count(&self) -> usize {
        let mut iter = self.individual_answers.iter();
        if let Some(first) = iter.next() {
            let mut intersection = first.clone();
            for answers in iter {
                intersection = intersection.intersection(answers).copied().collect();
            }
            intersection.len()
        } else {
            0
        }
    }
}



fn main() -> Result<()> {
    let input = read_input(6)?;

    let mut groups = Vec::new();
    for group_input in input.split("\n\n") {
        groups.push(GroupAnswers::parse(group_input)?);
    }

    let part_1_sum : usize = groups.iter().map(|group| group.distinct_any_count()).sum();
    println!("Part 1 sum: {}", part_1_sum);

    let part_2_sum : usize = groups.iter().map(|group| group.distinct_all_count()).sum();
    println!("Part 2 sum: {}", part_2_sum);

    Ok(())
}