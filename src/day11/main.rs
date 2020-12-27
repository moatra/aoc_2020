use anyhow::{Result as AResult, bail};
use aoc_2020::read_input;
use std::fmt;
use std::fmt::Write;

#[derive(Clone, Debug)]
enum GridContents {
    Floor,
    OccupiedSeat,
    EmptySeat,
}

impl GridContents {
    fn is_occupied(&self) -> bool {
        match self {
            GridContents::OccupiedSeat => true,
            GridContents::EmptySeat => false,
            GridContents::Floor => false
        }
    }
}

#[derive(Clone)]
struct FerrySeating {
    height: usize,
    width: usize,
    data: Vec<GridContents>
}

impl FerrySeating {
    fn parse(input: String) -> AResult<FerrySeating> {
        let lines : Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().unwrap().len();
        let mut data = Vec::with_capacity(width * height);
        for line in lines.iter() {
            if line.len() != width {
                bail!("Non-square grid");
            }
            for c in line.chars() {
                data.push(match c {
                    '.' => GridContents::Floor,
                    'L' => GridContents::EmptySeat,
                    '#' => GridContents::OccupiedSeat,
                    other => bail!("Unrecognized grid entry {}", other)
                });
            }
        }
        Ok(FerrySeating {
            height,
            width,
            data
        })
    }

    fn next_state(&self) -> Option<FerrySeating> {
        let mut next_data = Vec::new();
        let mut changed = false;
        for (grid_pos, pos_state) in self.data.iter().enumerate() {
            let (pos_changed, next_pos_state) = self.next_pos_state(grid_pos, pos_state);
            changed = changed || pos_changed;
            next_data.push(next_pos_state);
        }
        if changed {
            Some(FerrySeating {
                height: self.height,
                width: self.width,
                data: next_data
            })
        } else {
            None
        }
    }
    fn next_pos_state(&self, grid_pos: usize, cur_state: &GridContents) -> (bool, GridContents) {
        // todo: consume old state so we don't need to re-allocate
        let adjacent = self.adjacent_iter(grid_pos);
        let adj_occupied_count = adjacent.iter()
            .filter(|pos| pos.is_occupied())
            .count();
        let ret = match cur_state {
            GridContents::OccupiedSeat if adj_occupied_count >= 4 => (true, GridContents::EmptySeat),
            GridContents::EmptySeat if adj_occupied_count == 0 => (true, GridContents::OccupiedSeat),
            state => (false, state.clone()),
        };
        // println!("Evaluating pos {} {:?}, cur {:?}, next: {:?}, adjcnt: {}, adjacent: {:?}", grid_pos, self.to_coords(grid_pos), cur_state, ret, adj_occupied_count, adjacent);
        ret

    }

    // fn adjacent_iter<'a>(&'a self, grid_pos: usize) -> impl Iterator<Item = &GridContents> + 'a {
    //     // todo: custom iterator implementation that tracks 0-8 and grid_pos to impl next
    //     let (x, y) = self.to_coords(grid_pos);
    //     [
    //         self.to_grid_pos(x - 1, y - 1), self.to_grid_pos(x, y - 1), self.to_grid_pos(x + 1, y - 1),
    //         self.to_grid_pos(x - 1, y),                                       self.to_grid_pos(x + 1, y),
    //         self.to_grid_pos(x - 1, y + 1), self.to_grid_pos(x, y + 1), self.to_grid_pos(x + 1, y + 1),
    //     ].iter().copied().map(move |pos| self.data.get(pos)).flatten()
    // }
    fn adjacent_iter<'a>(&'a self, grid_pos: usize) -> Vec<&'a GridContents> {
        // todo: custom iterator implementation that tracks 0-8 and grid_pos to impl next
        let (x, y) = self.to_coords(grid_pos);
        let adjacent_positions : [Option<usize>; 8] = [
            self.to_grid_pos(x - 1, y - 1), self.to_grid_pos(x, y - 1), self.to_grid_pos(x + 1, y - 1),
            self.to_grid_pos(x - 1, y),                                       self.to_grid_pos(x + 1, y),
            self.to_grid_pos(x - 1, y + 1), self.to_grid_pos(x, y + 1), self.to_grid_pos(x + 1, y + 1),
        ];

        // todo: why does .flatten().map(...) work, but .flat_map(...) did not?
        adjacent_positions.iter().flatten().map(move |pos| self.data.get(*pos)).flatten().collect()
    }

    fn to_coords(&self, grid_pos: usize) -> (isize, isize) {
        let x = (grid_pos % self.width) as isize;
        let y = (grid_pos / self.width) as isize;
        (x, y)
    }

    fn to_grid_pos(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 {
            None
        } else {
            Some((y as usize * (self.width)) + (x as usize))
        }
    }
}

impl fmt::Display for FerrySeating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut col_count = 0;
        for cell in self.data.iter() {
            f.write_char(match cell {
                GridContents::Floor => '.',
                GridContents::OccupiedSeat => '#',
                GridContents::EmptySeat => 'L'
            })?;
            col_count += 1;
            if col_count == self.width {
                f.write_char('\n')?;
                col_count = 0;
            }
        }
        Ok(())
    }
}



fn main() -> AResult<()> {
    let mut part1 = FerrySeating::parse(read_input(11)?)?;
    println!("{}", part1);

    loop {
        match part1.next_state() {
            Some(next_iter) => part1 = next_iter,
            None => break
        }
    }

    // wtf - scoping is that it's not iteratively updating part1, it's always recalculating from the initial state
    // let mut counter = 0;
    // while let Some(part1) = part1.next_state() {
    //     println!("{}", part1);
    //     counter += 1;
    //     if counter > 5 {
    //         break;
    //     }
    // };

    let part1_answer = part1.data.iter().filter(|pos| pos.is_occupied()).count();
    println!("After stabilizing, {} seats are occupied", part1_answer);


    Ok(())
}