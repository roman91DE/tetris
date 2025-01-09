#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
    pub fn get_x(&self) -> u32 {
        self.x
    }
    pub fn get_y(&self) -> u32 {
        self.y
    }
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.get_x() + other.get_x(),
            y: self.get_y() + other.get_y(),
        }
    }
}
