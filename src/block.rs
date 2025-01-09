#![allow(unused)]
use crate::point::Point;
use arrayvec::ArrayVec;
use std::error::Error;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum BlockShape {
    Square,
    Line,
    T,
    L,
    LRev,
    Z,
    ZRev,
}

#[derive(Clone)]
pub struct Block {
    pub shape: BlockShape,
    pub coordinates: ArrayVec<Point, 4>,
}

#[derive(Debug)]
struct BlockMoveError {
    message: String,
}

impl fmt::Display for BlockMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BlockMoveError {}

impl Block {
    pub fn new(origin: Point, shape: BlockShape) -> Block {
        // Define relative coordinates for each BlockShape
        let relative_coordinates = match shape {
            BlockShape::Square => [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
            BlockShape::Line => [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ],
            BlockShape::T => [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
            BlockShape::L => [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(1, 2),
            ],
            BlockShape::LRev => [
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(1, 2),
                Point::new(0, 2),
            ],
            BlockShape::Z => [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(1, 0),
                Point::new(2, 0),
            ],
            BlockShape::ZRev => [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(2, 1),
            ],
        };
        let coordinates = relative_coordinates
            .iter()
            .map(|p| p.add(&origin))
            .collect();

        Block { shape, coordinates }
    }

    /// Rotates the block 90 degrees clockwise around its geometric center
    pub fn rotate(&self) -> Result<Block, Box<dyn Error>> {
        if self.shape == BlockShape::Square {
            return Ok(self.clone());
        }

        // Calculate the geometric center of the block
        let center_x = self.coordinates.iter().map(|p| p.get_x()).sum::<i32>() as f32
            / self.coordinates.len() as f32;
        let center_y = self.coordinates.iter().map(|p| p.get_y()).sum::<i32>() as f32
            / self.coordinates.len() as f32;

        let mut new_coordinates = ArrayVec::new();

        // Perform rotation for each point
        for coord in self.coordinates.iter() {
            // Translate point to the geometric center
            let relative_x = coord.get_x() as f32 - center_x;
            let relative_y = coord.get_y() as f32 - center_y;

            // Apply 90-degree clockwise rotation
            let rotated_x = center_x - relative_y;
            let rotated_y = center_y + relative_x;

            // Translate back and round to the nearest integer
            new_coordinates.try_push(Point::new(
                rotated_x.round() as i32,
                rotated_y.round() as i32,
            ))?;
        }

        let new_block = Block {
            shape: self.shape,
            coordinates: new_coordinates,
        };

        if new_block.not_negative() {
            Ok(new_block)
        } else {
            Err(Box::new(BlockMoveError {
                message: "Rotation resulted in negative coordinates".to_string(),
            }))
        }
    }

    pub fn not_negative(&self) -> bool {
        self.coordinates.iter().all(|f| f.not_negative())
    }

    pub fn push_down(&self) -> Result<Block, Box<dyn Error>> {
        let drop_point = Point::new(0, 1);
        let new_block = Block {
            shape: self.shape,
            coordinates: self
                .coordinates
                .iter()
                .map(|p| p.add(&drop_point))
                .collect(),
        };
        if new_block.not_negative() {
            Ok(new_block)
        } else {
            Err(Box::new(BlockMoveError {
                message: "Push-Down resulted in negative coordinates".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_block_initialization() {
        let origin = Point::new(0, 0);
        let block = Block::new(origin, BlockShape::Square);

        let expected_coordinates = [
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];
        assert_eq!(block.coordinates.as_slice(), &expected_coordinates);
    }

    #[test]
    fn test_line_block_initialization() {
        let origin = Point::new(5, 5);
        let block = Block::new(origin, BlockShape::Line);

        let expected_coordinates = [
            Point::new(5, 5),
            Point::new(6, 5),
            Point::new(7, 5),
            Point::new(8, 5),
        ];
        assert_eq!(block.coordinates.as_slice(), &expected_coordinates);
    }

    #[test]
    fn test_block_rotation_square() {
        let origin = Point::new(4, 4);
        let block = Block::new(origin, BlockShape::Square);
        let rotated_block = block.rotate();

        // Square blocks should remain unchanged after rotation
        assert_eq!(block.coordinates, rotated_block.unwrap().coordinates);
    }

    #[test]
    fn test_block_pushdown_square() {
        let origin = Point::new(4, 4);
        let block = Block::new(origin, BlockShape::Square);
        let pushed_down = block.push_down().unwrap();

        let expected_coordinates = [
            Point::new(4, 5),
            Point::new(5, 5),
            Point::new(4, 6),
            Point::new(5, 6),
        ];

        // Square blocks should remain unchanged after rotation
        assert_eq!(pushed_down.coordinates.as_slice(), &expected_coordinates);
    }

    #[test]
    fn test_clone_behavior() {
        let origin = Point::new(0, 0);
        let block = Block::new(origin, BlockShape::T);
        let cloned_block = block.clone();

        assert_eq!(block.coordinates, cloned_block.coordinates);
        assert_eq!(block.shape, cloned_block.shape);
    }
}
