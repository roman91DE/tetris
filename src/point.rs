#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.get_x() + other.get_x(),
            y: self.get_y() + other.get_y(),
        }
    }
    pub fn not_negative(&self) -> bool {
        self.get_x() >= 0 && self.get_y() >= 0
    }
}
