use anyhow::{Result as AResult, bail};
use std::convert::TryFrom;
use aoc_2020::parse_input;

// todo: refactor to using actual vector & trig math

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}
impl TryFrom<i32> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value % 360 {
            -270 => Ok(Direction::East),
            -180 => Ok(Direction::South),
            -90 => Ok(Direction::West),
            0 => Ok(Direction::North),
            90 => Ok(Direction::East),
            180 => Ok(Direction::South),
            270 => Ok(Direction::West),
            _ => bail!("Unknown direction angle")
        }
    }
}

impl Direction {
    fn rotate(&self, rot: &Rotation, qty: &RotationQty) -> Direction {
        let facing = match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        };
        let rotation_mod = match rot {
            Rotation::Right => 1,
            Rotation::Left => -1,
        };
        let rotation_qty = match qty {
            RotationQty::R0 => 0,
            RotationQty::R90 => 90,
            RotationQty::R180 => 180,
            RotationQty::R270 => 270,
        };

        let next_facing = (facing + (rotation_mod * rotation_qty)) % 360;

        Direction::try_from(next_facing).expect("Invalid rotation math")
    }
}


#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum RotationQty {
    R0,
    R90,
    R180,
    R270,
}

impl TryFrom<i32> for RotationQty {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RotationQty::R0),
            90 => Ok(RotationQty::R90),
            180 => Ok(RotationQty::R180),
            270 => Ok(RotationQty::R270),
            _ => bail!("Unknown rotation quantity")
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Translate(Direction, i32),
    Turn(Rotation, RotationQty),
    Advance(i32),
}

impl Instruction {
    fn parse(input: &str) -> AResult<Instruction> {
        let qty: i32 = input.chars().skip(1).collect::<String>().parse()?;
        match input.chars().nth(0) {
            Some('N') => Ok(Instruction::Translate(Direction::North, qty)),
            Some('E') => Ok(Instruction::Translate(Direction::East, qty)),
            Some('S') => Ok(Instruction::Translate(Direction::South, qty)),
            Some('W') => Ok(Instruction::Translate(Direction::West, qty)),
            Some('L') => Ok(Instruction::Turn(Rotation::Left, RotationQty::try_from(qty)?)),
            Some('R') => Ok(Instruction::Turn(Rotation::Right, RotationQty::try_from(qty)?)),
            Some('F') => Ok(Instruction::Advance(qty)),
            _ => bail!("Unexpected instruction")
        }
    }
}

#[derive(Debug, Clone)]
struct ShipState {
    x: i32,
    y: i32,
    facing: Direction
}

impl ShipState {
    fn apply(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Translate(Direction::North, qty) => self.y -= qty,
            Instruction::Translate(Direction::East, qty) => self.x += qty,
            Instruction::Translate(Direction::South, qty) => self.y += qty,
            Instruction::Translate(Direction::West, qty) => self.x -= qty,

            Instruction::Turn(rot, qty) => self.facing = self.facing.rotate(rot, qty),
            Instruction::Advance(qty) => self.apply(&Instruction::Translate(self.facing, *qty))
        }
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone)]
struct ShipAndWaypointState {
    ship: ShipState,
    waypoint: (i32, i32)
}

impl ShipAndWaypointState {
    fn apply(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Translate(Direction::North, qty) => self.waypoint.1 -= qty,
            Instruction::Translate(Direction::East, qty) => self.waypoint.0 += qty,
            Instruction::Translate(Direction::South, qty) => self.waypoint.1 += qty,
            Instruction::Translate(Direction::West, qty) => self.waypoint.0 -= qty,

            Instruction::Turn(_, RotationQty::R0) => {},
            Instruction::Turn(_, RotationQty::R180) => self.set_waypoint(-1 * self.waypoint.0, -1 * self.waypoint.1),

            Instruction::Turn(Rotation::Right, RotationQty::R90) => self.set_waypoint(-1 * self.waypoint.1, self.waypoint.0),
            Instruction::Turn(Rotation::Left, RotationQty::R270) => self.set_waypoint(-1 * self.waypoint.1, self.waypoint.0),

            Instruction::Turn(Rotation::Right, RotationQty::R270) => self.set_waypoint(self.waypoint.1, -1 * self.waypoint.0),
            Instruction::Turn(Rotation::Left, RotationQty::R90)   => self.set_waypoint(self.waypoint.1, -1 * self.waypoint.0),

            Instruction::Advance(qty) => {
                self.ship.x += qty * self.waypoint.0;
                self.ship.y += qty * self.waypoint.1;
            }
        }
    }
    fn set_waypoint(&mut self, x: i32, y: i32) {
        self.waypoint = (x, y);
    }
}

fn main() -> AResult<()> {
    let input = parse_input(12, |line| Instruction::parse(line))?;

    let init_state = ShipState {
        x: 0,
        y: 0,
        facing: Direction::East
    };

    let mut current_state = init_state.clone();
    for instr in input.iter() {
        current_state.apply(instr);
    }

    println!("Part 1: {}", current_state.manhattan());

    let init_state = ShipAndWaypointState {
        ship: init_state.clone(),
        waypoint: (10, -1)
    };

    let mut current_state = init_state.clone();
    // println!("{:?}", current_state);
    for instr in input.iter() {
        // println!("applying {:?}", instr);
        current_state.apply(instr);
        // println!("{:?}", current_state);
    }

    println!("Part 2: {}", current_state.ship.manhattan());


    Ok(())
}