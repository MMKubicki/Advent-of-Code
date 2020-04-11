use crate::util::point::Point;
use std::str::FromStr;

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Wire {
    points: Vec<Point>,
}

impl Wire {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn get_crosses(&self, other: &Wire) -> Vec<Point> {
        let mut points = Vec::new();

        println!(
            "Checking {}*{} Points",
            self.points.len(),
            other.points.len()
        );

        for this_point in self.get_points() {
            for other_point in other.get_points() {
                if this_point == other_point {
                    points.push(this_point.clone());
                }
            }
        }

        points
    }

    pub fn get_length_to_point(&self, point: &Point) -> Option<usize> {
        match self.points.iter().position(|p| p == point) {
            Some(val) => Some(val + 1),
            None => None,
        }
    }
}

#[derive(Error, Debug, Clone)]
#[error("Failed to parse into Wire: {source}")]
pub struct ParseWireError {
    source: ParseIntError,
}

impl FromStr for Wire {
    type Err = ParseWireError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();

        enum Dir {
            Up,
            Down,
            Left,
            Right,
        }

        let orders = s
            .split(',')
            .map(|s: &str| {
                let mut chars = s.chars();
                let dir = match chars.next().unwrap_or('U') {
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => Dir::Up,
                };

                let dist: i32 = chars.collect::<String>().parse::<i32>().unwrap_or(0);

                (dir, dist)
            })
            .collect::<Vec<_>>();

        let mut pt = Point::default();
        for (dir, distance) in orders {
            for _ in 1..(distance + 1) {
                let new_pt = pt
                    + match dir {
                        Dir::Up => Point::new(0, 1),
                        Dir::Down => Point::new(0, -1),
                        Dir::Left => Point::new(-1, 0),
                        Dir::Right => Point::new(1, 0),
                    };
                points.push(new_pt);
                pt = new_pt;
            }
        }

        Ok(Self::new(points))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wire_without_err() {
        assert!("R8,U5,L5,D3".parse::<Wire>().is_ok());
    }

    #[test]
    fn parse_wire_correctly() {
        let wire = "R4,U2,L2,D1".parse::<Wire>().expect("Failed parsing Wire");

        let vec = vec![
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(2, 1),
        ];
        assert_eq!(wire.points, vec);
    }
}
