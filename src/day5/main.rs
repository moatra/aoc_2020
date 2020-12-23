use aoc_2020::parse_input;
use anyhow::{bail, Result};
use std::collections::HashSet;

struct SeatPosition {
    row: usize,
    col: usize,
}

impl SeatPosition {
    fn seat_id(&self) -> usize {
        (self.row * 8) + self.col
    }

    fn parse(instr: &str) -> Result<SeatPosition> {
        if instr.len() != 10 {
            bail!("Invalid seat encoding length")
        }
        let mut row = 0;
        for  c in instr.chars().take(7) {
            row *= 2;
            if c == 'B' {
                row += 1
            } else if c != 'F' {
                bail!("Invalid seat encoding row value");
            }
        };
        let mut col = 0;
        for c in instr.chars().skip(7) {
            col *= 2;
            if c == 'R' {
                col += 1;
            } else if c != 'L' {
                bail!("Invalid seat encoding col value");
            }
        };
        Ok(SeatPosition { row, col })
    }
}

fn main() -> Result<()> {
    let seats = parse_input(5, |line| SeatPosition::parse(line))?;

    let max_id = seats.iter().map(|seat| seat.seat_id()).max().unwrap();
    println!("Max seat id: {}", max_id);

    let min_id = seats.iter().map(|seat| seat.seat_id()).min().unwrap();

    let scanned_ids : HashSet<usize> = seats.iter().map(|seat| seat.seat_id()).collect();
    for id in min_id ..= max_id {
        if !scanned_ids.contains(&id) {
            println!("Missing id: {}", id);
        }
    }

    Ok(())
}