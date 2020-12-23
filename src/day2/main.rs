use aoc_2020::parse_input;
use anyhow::Result;
use itertools::Itertools;
use std::convert::TryFrom;
use std::ops::{RangeInclusive};

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    repeated_char: char,
    password: String,
}



impl PasswordPolicy {
    fn acceptable_counts(&self) -> RangeInclusive<usize> {
        self.min ..= self.max
    }

    fn is_valid(&self) -> bool {
        self.acceptable_counts().contains(
            &self.password.chars().filter(|c| c == &self.repeated_char).count()
        )
    }
    fn is_valid_at_new_job(&self) -> bool {
       match (self.check_pos(self.min), self.check_pos(self.max)) {
           (true, false) => true,
           (false, true) => true,
           _ => false
       }
    }

    fn check_pos(&self, pos: usize) -> bool {
        match self.password.chars().nth(pos - 1) {
            Some(c) if c == self.repeated_char => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for PasswordPolicy {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split(['-', ' ', ':'].as_ref()).filter(|s| !s.is_empty()).collect_vec().as_slice() {
            &[min, max, repeat, password] => {
                let min = min.parse()?;
                let max = max.parse()?;
                if min < max && repeat.len() == 1 && password.len() > 0 {
                    Ok(PasswordPolicy {
                        min,
                        max,
                        repeated_char: repeat.chars().nth(0).unwrap(), // length asserted in if
                        password: password.to_string(),
                    })
                } else {
                    Err(anyhow::anyhow!("Invalid input semantics"))
                }
            }
            _ => Err(anyhow::anyhow!("Invalid input format"))
        }
    }
}

fn main() -> Result<()> {
    let input = parse_input(2, |line| PasswordPolicy::try_from(line))?;

    let matches = input.iter().filter(|line| line.is_valid()).count();

    println!("Found {} acceptable passwords", matches);

    let new_matches = input.iter().filter(|line| line.is_valid_at_new_job()).count();

    println!("Found {} acceptable passwords at new job", new_matches);

    Ok(())
}
