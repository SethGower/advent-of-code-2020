use std::ops::Mul;
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
}
impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Ship {
    coords: Point,
    waypoint: Point,
}
impl Ship {
    fn rotate_right(&mut self) {
        std::mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
        self.waypoint.y *= -1;
    }
    fn rotate_left(&mut self) {
        std::mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
        self.waypoint.x *= -1;
    }
    fn new(position: Point, waypoint: Point) -> Ship {
        Ship {
            coords: position,
            waypoint: waypoint,
        }
    }
    fn travel(&mut self, motion: Motion) {
        match motion {
            Motion::N(dist) => {
                self.waypoint.translate(0, dist);
            }
            Motion::S(dist) => {
                self.waypoint.translate(0, -1 * dist);
            }
            Motion::E(dist) => {
                self.waypoint.translate(dist, 0);
            }
            Motion::W(dist) => {
                self.waypoint.translate(-1 * dist, 0);
            }
            Motion::L(dist) => {
                for _ in 0..dist / 90 {
                    self.rotate_left();
                }
            }
            Motion::R(dist) => {
                for _ in 0..dist / 90 {
                    self.rotate_right();
                }
            }
            Motion::F(dist) => {
                let vec = self.waypoint * dist;
                self.coords.translate(vec.x, vec.y);
            }
        }
    }
    fn manhattan_distance(&self, origin: Point) -> (i32, i32) {
        (
            (self.coords.x - origin.x).abs(),
            (self.coords.y - origin.y).abs(),
        )
    }
}
pub fn part1(input: String) -> Option<String> {
    let motions: Vec<Motion> = input.lines().map(|x| Motion::parse(x).unwrap()).collect();
    let mut ship = Ship::new(Point { x: 0, y: 0 }, Point { x: 10, y: 1 });
    for motion in motions.iter() {
        ship.travel(*motion);
    }
    let (x, y) = ship.manhattan_distance(Point { x: 0, y: 0 });
    println!("{:#?}", ship);
    println!("{}", x + y);

    Some((x + y).to_string())
}
