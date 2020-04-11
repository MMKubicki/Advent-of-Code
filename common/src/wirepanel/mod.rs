pub mod wire;

use crate::util::point::Point;
use crate::wirepanel::wire::ParseWireError;
use std::str::FromStr;
use thiserror::Error;
use wire::Wire;

#[derive(Clone, Debug)]
pub struct Panel {
    wires: [Wire; 2],
}

impl Panel {
    pub fn new(wires: [Wire; 2]) -> Self {
        Self { wires }
    }

    pub fn get_crosses(&self) -> CrossedPoints {
        let crosses = self.wires[0].get_crosses(&self.wires[1]);
        CrossedPoints::new(self, crosses)
    }
}

#[derive(Error, Debug, Clone)]
pub enum ParsePanelError {
    #[error("Wrong count of lines. Expected: 2 Found: {0}")]
    WireCount(usize),
    #[error(transparent)]
    ParseWireError(#[from] ParseWireError),
}

impl FromStr for Panel {
    type Err = ParsePanelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.len() != 2 {
            return Err(ParsePanelError::WireCount(lines.len()));
        }

        let wire1 = lines[0].parse::<Wire>()?;
        let wire2 = lines[1].parse::<Wire>()?;

        Ok(Panel::new([wire1, wire2]))
    }
}

#[derive(Debug, Clone)]
pub struct CrossedPoints<'a> {
    panel: &'a Panel,
    points: Vec<Point>,
}

impl<'a> CrossedPoints<'a> {
    pub fn new(panel: &'a Panel, points: Vec<Point>) -> Self {
        Self { panel, points }
    }

    pub fn get_nearest_by_distance(&self) -> (&Point, usize) {
        let mut temp = self
            .points
            .iter()
            .map(|p| (p, p.manhatten_distance(Default::default())))
            .collect::<Vec<(&Point, usize)>>();
        temp.sort_by(|(_, d), (_, d2)| d.cmp(d2));
        temp[0]
    }

    pub fn get_nearest_by_wire_length(&self) -> (&Point, usize) {
        let mut temp = self
            .points
            .iter()
            .map(|p| {
                (
                    p,
                    self.panel.wires[0].get_length_to_point(p).unwrap()
                        + self.panel.wires[1].get_length_to_point(p).unwrap(),
                )
            })
            .collect::<Vec<(&Point, usize)>>();
        temp.sort_by(|(_, d), (_, d2)| d.cmp(d2));
        temp[0]
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_parse_without_err() {
        assert!(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
                .parse::<Panel>()
                .is_ok()
        )
    }

    #[test]
    fn panel_test1() {
        let panel = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            .parse::<Panel>()
            .expect("Failed parsing Panel");

        assert_eq!(panel.get_crosses().get_nearest_by_distance().1, 159)
    }

    #[test]
    fn panel_test2() {
        let panel =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                .parse::<Panel>()
                .expect("Failed parsing Panel");

        assert_eq!(panel.get_crosses().get_nearest_by_distance().1, 135)
    }

    #[test]
    fn panel_test_length1() {
        let panel = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            .parse::<Panel>()
            .expect("Failed parsing Panel");

        assert_eq!(panel.get_crosses().get_nearest_by_wire_length().1, 610)
    }

    #[test]
    fn panel_test_length2() {
        let panel =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                .parse::<Panel>()
                .expect("Failed parsing Panel");

        assert_eq!(panel.get_crosses().get_nearest_by_wire_length().1, 410)
    }
}
