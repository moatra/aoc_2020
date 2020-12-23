use anyhow::{Result as AResult};
use thiserror::Error;
use std::collections::{VecDeque, HashMap};
use aoc_2020::parse_input;

#[derive(Error, Debug)]
enum ConsumptionError {
    #[error("value {0} is not a sum in the current window")]
    InvalidValue(i128)
}

struct XmasPreamble {
    window_size: usize,
    window: VecDeque<i128>,
    sums: HashMap<i128, usize>
}
impl XmasPreamble {
    fn new(size: usize) -> XmasPreamble {
        let mut preamble = VecDeque::with_capacity(size);
        XmasPreamble {
            window_size: size,
            window: preamble,
            sums: HashMap::with_capacity(size * size)
        }
    }
    fn consume(&mut self, next: i128) -> Result<(), ConsumptionError> {
        if self.window.len() != self.window_size {
            self.increment_sums(next);
            self.window.push_back(next);
            Ok(())
        } else {
            if !self.sums.contains_key(&next) {
                Err(ConsumptionError::InvalidValue(next))
            } else {
                let to_remove = self.window.pop_front().expect("Broken invariant");
                self.decrement_sums(to_remove);
                self.increment_sums(next);
                self.window.push_back(next);
                Ok(())
            }
        }
    }

    fn increment_sums(&mut self, next: i128) {
        for value in self.window.iter() {
            if *value != next {
                let mut pair_count = self.sums.entry(value + next).or_insert(0);
                *pair_count += 1;
            }
        }
    }
    fn decrement_sums(&mut self, old: i128) {
        for value in self.window.iter() {
            if *value != old {
                let sum = value + old;
                let mut pair_count = self.sums.get_mut(&sum).expect("Broken invariant");
                *pair_count -= 1;
                if *pair_count == 0 {
                    self.sums.remove(&sum);
                }
            }
        }
    }
}

fn part1(numbers: &Vec<i128>) -> Result<(), ConsumptionError> {
    let mut preamble = XmasPreamble::new(25);

    for number in numbers {
        preamble.consume(*number)?;
    }
    Ok(())
}
fn part2(search: &[i128], sum: i128) -> Option<i128> {
    let mut cur = 0;
    // todo: efficiency
    for start in 0 .. (search.len() - 1) {
        for end in (start + 1) .. search.len() {
            let sub = &search[start ..= end];
            if sub.iter().sum::<i128>() == sum {
                let min = sub.iter().min().unwrap();
                let max = sub.iter().max().unwrap();
                return Some(min + max)
            }
        }
    }
    None
}

fn main() -> AResult<()> {
    let numbers = parse_input(9, |line| line.parse::<i128>())?;

    if let Err(ConsumptionError::InvalidValue(value)) = part1(&numbers) {
        // search everything up to the value; any range including the value (todo: or after the value) will sum too high
        let err_value_idx = numbers.iter().position(|cur| *cur == value).expect("Broken invariant");
        let part2 = part2(&numbers[0 .. err_value_idx], value).unwrap();
        println!("part2: {}", part2)
    }

    Ok(())
}