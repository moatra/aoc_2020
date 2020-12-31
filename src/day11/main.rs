use anyhow::{Result as AResult, bail};
use aoc_2020::read_input;
use std::fmt;
use std::fmt::Write;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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
    fn is_seat(&self) -> bool {
        match self {
            GridContents::OccupiedSeat => true,
            GridContents::EmptySeat => true,
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum CursorDirection {
    Id,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct FerryIndex {
    x: usize,
    y: usize,
}

impl FerryIndex {
    fn id(&self, grid: &FerrySeating) -> usize {
        (self.y * grid.width) + self.x
    }
    fn get<'a>(&self, grid: &'a FerrySeating) -> Option<&'a GridContents> {
        // <0 impossible to represent thanks to unsigned
        if self.x >= grid.width || self.y >= grid.height {
            None
        } else {
            grid.data.get(self.id(grid))
        }
    }

    // todo: simplify
    fn advance(&self, grid: &FerrySeating, dir: CursorDirection) -> Option<FerryIndex> {
        match dir {
            CursorDirection::Id => {
                let id = self.id(grid) + 1;
                if id < grid.data.len() {
                    Some(FerryIndex {
                        x: id % grid.width,
                        y: id / grid.width
                    })
                } else {
                    None
                }
            }
            CursorDirection::N => {
                if self.y == 0 {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x,
                        y: self.y - 1
                    })
                }
            }
            CursorDirection::NE => {
                if self.y == 0 || self.x + 1 >= grid.width {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x + 1,
                        y: self.y - 1
                    })
                }
            }
            CursorDirection::E => {
                if self.x + 1 >= grid.width {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x + 1,
                        y: self.y
                    })
                }
            }
            CursorDirection::SE => {
                if self.y + 1 >= grid.height || self.x + 1 >= grid.width {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x + 1,
                        y: self.y + 1
                    })
                }
            }
            CursorDirection::S => {
                if self.y + 1 >= grid.height {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x,
                        y: self.y + 1
                    })
                }
            }
            CursorDirection::SW => {
                if self.y + 1 >= grid.height || self.x == 0 {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x - 1,
                        y: self.y + 1
                    })
                }
            }
            CursorDirection::W => {
                if self.x == 0 {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x - 1,
                        y: self.y
                    })
                }
            }
            CursorDirection::NW => {
                if self.x == 0 || self.y == 0 {
                    None
                } else {
                    Some(FerryIndex {
                        x: self.x - 1,
                        y: self.y - 1
                    })
                }
            }
        }
    }
}

struct FerryCursor<'a> {
    grid: &'a FerrySeating,
    curr: Option<FerryIndex>,
    dir: CursorDirection
}

impl Iterator for FerryCursor<'_> {
    type Item = (FerryIndex, GridContents);

    fn next(&mut self) -> Option<Self::Item> {
        let results = self.curr.and_then(|indx| indx.get(self.grid).map(|data| (indx, *data)));

        self.curr = self.curr.and_then(|idx| idx.advance(self.grid, self.dir));

        results
    }
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

    fn iter(&self) -> FerryCursor {
        FerryCursor {
            grid: self,
            curr: Some(FerryIndex { x: 0, y: 0 }),
            dir: CursorDirection::Id
        }
    }

    fn search_iter(&self, from: FerryIndex, dir: CursorDirection) -> FerryCursor {
        FerryCursor {
            grid: self,
            curr: from.advance(self, dir),
            dir
        }
    }

    fn next_state(&self, logic: &Box<dyn SeatingLogic>) -> Option<FerrySeating> {
        let mut next_data = Vec::new();
        let mut changed = false;
        for (idx, pos_state) in self.iter() {
            let next_pos_state = logic.next_pos_state(self, idx, pos_state);
            changed = changed || (pos_state != next_pos_state);
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

trait SeatingLogic {
    // todo: remove self logic?  maybe by just using function pointers?
    fn next_pos_state(&self, grid: &FerrySeating, idx: FerryIndex, cur_state: GridContents) -> GridContents;
}

struct Pt1Logic {}

impl Pt1Logic {
    fn adjacent_iter(grid: &FerrySeating, idx: FerryIndex) -> Vec<(FerryIndex, GridContents)> {
        vec![
            grid.search_iter(idx, CursorDirection::N),
            grid.search_iter(idx, CursorDirection::NE),
            grid.search_iter(idx, CursorDirection::E),
            grid.search_iter(idx, CursorDirection::SE),
            grid.search_iter(idx, CursorDirection::S),
            grid.search_iter(idx, CursorDirection::SW),
            grid.search_iter(idx, CursorDirection::W),
            grid.search_iter(idx, CursorDirection::NW),
        ].iter_mut()
            .map(|iter| iter.next())
            .flatten()
            .collect() // todo: skip collect?
    }

}

fn common_decider(cur_state: GridContents, adjacent: Vec<(FerryIndex, GridContents)>, visible_req: usize) -> GridContents {
    let adj_occupied_count = adjacent.iter()
        .filter(|(_grid_pos, pos)| pos.is_occupied())
        .count();
    let ret = match cur_state {
        GridContents::OccupiedSeat if adj_occupied_count >= visible_req => GridContents::EmptySeat,
        GridContents::EmptySeat if adj_occupied_count == 0 => GridContents::OccupiedSeat,
        state => state,
    };
    // println!("Evaluating pos {} {:?}, cur {:?}, next: {:?}, adjcnt: {}, adjacent: {:?}", grid_pos, self.to_coords(grid_pos), cur_state, ret, adj_occupied_count, adjacent);
    ret
}

struct SimulationResults {
    iterations: usize,
    occupied: usize
}
fn common_simulator(starting_grid: FerrySeating, logic: Box<dyn SeatingLogic>) -> SimulationResults {
    let mut current_grid = starting_grid;
    let mut iteration_count = 0;
    // println!("{}", current_grid);
    // println!();

    loop {
        match current_grid.next_state(&logic) {
            Some(next_iter) => {
                current_grid = next_iter;
                iteration_count += 1;
                // println!("round {}:", iteration_count);
                // println!("{}", current_grid);
                // println!();
            },
            None => break
        }
    }

    SimulationResults {
        iterations: iteration_count,
        occupied: current_grid.data.iter().filter(|pos| pos.is_occupied()).count()
    }
}

impl SeatingLogic for Pt1Logic {
    fn next_pos_state(&self, grid: &FerrySeating, grid_pos: FerryIndex, cur_state: GridContents) -> GridContents {
        common_decider(cur_state, Pt1Logic::adjacent_iter(grid, grid_pos), 4)
    }
}

struct Pt2Logic {}

impl Pt2Logic {
    fn adjacent_iter(grid: &FerrySeating, idx: FerryIndex) -> Vec<(FerryIndex, GridContents)> {
        vec![
            grid.search_iter(idx, CursorDirection::N),
            grid.search_iter(idx, CursorDirection::NE),
            grid.search_iter(idx, CursorDirection::E),
            grid.search_iter(idx, CursorDirection::SE),
            grid.search_iter(idx, CursorDirection::S),
            grid.search_iter(idx, CursorDirection::SW),
            grid.search_iter(idx, CursorDirection::W),
            grid.search_iter(idx, CursorDirection::NW),
        ].iter_mut()
            .map(|iter| iter.find(|(_idx, pos)| pos.is_seat()))
            .flatten()
            .collect() // todo: skip collect?
    }
}

impl SeatingLogic for Pt2Logic {
    fn next_pos_state(&self, grid: &FerrySeating, grid_pos: FerryIndex, cur_state: GridContents) -> GridContents {
        common_decider(cur_state, Pt2Logic::adjacent_iter(grid, grid_pos), 5)
    }
}



fn main() -> AResult<()> {
    let input = FerrySeating::parse(read_input(11)?)?;

    // wtf - scoping is that it's not iteratively updating part1, it's always recalculating from the initial state
    // let mut counter = 0;
    // while let Some(part1) = part1.next_state() {
    //     println!("{}", part1);
    //     counter += 1;
    //     if counter > 5 {
    //         break;
    //     }
    // };

    let part1_answer = common_simulator(input.clone(), Box::new(Pt1Logic {}));
    println!("Stabilized after {} rounds, {} seats are occupied", part1_answer.iterations, part1_answer.occupied);

    let part2_answer = common_simulator(input, Box::new(Pt2Logic {}));
    println!("Stabilized after {} rounds, {} seats are occupied", part2_answer.iterations, part2_answer.occupied);


    Ok(())
}