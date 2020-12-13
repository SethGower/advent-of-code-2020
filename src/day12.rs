#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Motion {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl Motion {
    fn parse(s: &str) -> Option<Motion> {
        let tup = (&s[..1], s[1..].parse::<i32>().ok()?);
        match tup {
            ("N", val) => Some(Motion::N(val)),
            ("S", val) => Some(Motion::S(val)),
            ("E", val) => Some(Motion::E(val)),
            ("W", val) => Some(Motion::W(val)),
            ("L", val) => Some(Motion::L(val)),
            ("R", val) => Some(Motion::R(val)),
            ("F", val) => Some(Motion::F(val)),
            _ => None,
        }
    }
}
impl Point {
    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
    fn change(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Ship {
    coords: Point,
    dir: Direction,
}
impl Ship {
    fn rotateRight(&mut self) {
        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn rotateLeft(&mut self) {
        self.dir = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn new(dir: Direction, position: Point) -> Ship {
        Ship {
            coords: position,
            dir: dir,
        }
    }
    fn travel(&mut self, motion: Motion) -> Point {
        let point = self.coords.clone();
        match motion {
            Motion::N(dist) => {
                self.coords.translate(0, dist);
            }
            Motion::S(dist) => {
                self.coords.translate(0, -1 * dist);
            }
            Motion::E(dist) => {
                self.coords.translate(dist, 0);
            }
            Motion::W(dist) => {
                self.coords.translate(-1 * dist, 0);
            }
            Motion::L(dist) => {
                self.rotateLeft();
            }
            Motion::R(dist) => {
                self.rotateRight();
            }
            Motion::F(dist) => {}
        }
        point
    }
}
pub fn part1(input: String) -> Option<String> {
    let motions: Vec<Motion> = input.lines().map(|x| Motion::parse(x).unwrap()).collect();
    Some(input)
}
