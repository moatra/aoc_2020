use aoc_2020::read_input;
use anyhow::{bail, Result};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn down(&self, amount: usize) -> Coord {
        Coord { x: self.x, y: self.y + amount}
    }
    fn right(&self, amount: usize) -> Coord {
        Coord { x: self.x + amount, y: self.y }
    }
}

struct TreeMap {
    trees: HashSet<Coord>,
    map_width: usize,
    map_height: usize
}

impl TreeMap {
    fn parse(input: &String) -> Result<TreeMap> {
        let mut lines = input.lines().peekable();
        let map_width = lines.peek().expect("Input had no lines").len();
        let mut tree_coords = HashSet::new();
        let mut map_height: usize = 0;
        for (y, line) in lines.enumerate() {
            map_height += 1;
            if line.len() != map_width {
                bail!("Line {} had non-standard width", y);
            }
            for (x, tile) in line.chars().enumerate() {
                if tile == '#' {
                    tree_coords.insert(Coord { x, y });
                }
            }
        }
        Ok(TreeMap {
            trees: tree_coords,
            map_width,
            map_height,
        })
    }

    // simulate infinitely repeating-horizontal map by modulating x by map width;
    fn is_tree(&self, coord: &Coord) -> bool {
        self.trees.contains(&Coord{ x: coord.x % self.map_width, y: coord.y })
    }

    fn trees_for_slope(&self, down: usize, right: usize) -> i32 {
        let mut current = Coord { x: 0, y: 0 };
        let mut trees_encountered = 0;
        while current.y < self.map_height {
            if self.is_tree(&current) {
                trees_encountered += 1;
            }
            current = current.down(down).right(right);
        }
        trees_encountered
    }
}

fn main() -> Result<()> {
    let input = read_input(3)?;
    let map = TreeMap::parse(&input)?;

    let part1 = map.trees_for_slope(1, 3);

    println!("Encountered {} trees", part1);

    let part2 = map.trees_for_slope(1, 1) * map.trees_for_slope(1, 3) *
        map.trees_for_slope(1, 5) * map.trees_for_slope(1, 7) * map.trees_for_slope(2, 1);

    println!("Part2 answer: {}", part2);




    Ok(())
}
