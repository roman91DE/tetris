use crate::block::{Block, BlockShape};
use crate::point::Point;

pub struct Board {
    pub x_dim: i32,
    pub y_dim: i32,
    pub filled: Vec<Point>,
}

impl Board {
    pub fn new(x_dim: usize, y_dim: usize) -> Board {
        Board {
            x_dim: x_dim as i32,
            y_dim: y_dim as i32,
            filled: Vec::new(),
        }
    }

    pub fn sort_filled(&mut self) {
        self.filled.sort();
    }

    pub fn clear_board(&mut self) {
        // Count how many points are in each row (y-coordinate)
        let mut row_counts = vec![0; self.y_dim as usize];
        for point in &self.filled {
            if point.get_y() >= 0 && point.get_y() < self.y_dim {
                row_counts[point.get_y() as usize] += 1;
            }
        }

        // Identify fully filled rows
        let mut completed_rows = Vec::new();
        for (y, &count) in row_counts.iter().enumerate() {
            if count == self.x_dim {
                completed_rows.push(y as i32);
            }
        }

        // Remove points in completed rows
        self.filled
            .retain(|point| !completed_rows.contains(&point.get_y()));

        // Shift points above cleared rows downward
        for point in &mut self.filled {
            let rows_above = completed_rows
                .iter()
                .filter(|&&row| row < point.get_y())
                .count() as i32;
            *point = Point::new(point.get_x(), point.get_y() - rows_above);
        }
    }

    pub fn block_touches(&self, block: &Block) -> bool {
        for block_point in &block.coordinates {
            for filled_point in &self.filled {
                let dx = (block_point.get_x() - filled_point.get_x()).abs();
                let dy = (block_point.get_y() - filled_point.get_y()).abs();

                // Check if the block point touches or overlaps a filled point
                if (dx == 0 && dy == 1) || (dx == 1 && dy == 0) || (dx == 0 && dy == 0) {
                    return true;
                }
            }
        }

        // Also check if any block point touches the bottom of the board
        for block_point in &block.coordinates {
            if block_point.get_y() >= self.y_dim {
                return true;
            }
        }

        false
    }

    /// Places a block on the board by adding its points to the filled vector
    pub fn place_block(&mut self, block: &Block) {
        for point in &block.coordinates {
            // Add each block's point to the filled vector
            self.filled.push(Point::new(point.get_x(), point.get_y()));
        }

        // Sort the filled points for easier processing (e.g., clearing rows)
        self.sort_filled();
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_clear_no_rows_filled() {
        let mut board = Board::new(4, 5);
        board.filled = vec![Point::new(0, 0), Point::new(1, 2), Point::new(3, 4)];

        board.clear_board();

        // No rows were cleared, so the filled points remain unchanged
        let expected = vec![Point::new(0, 0), Point::new(1, 2), Point::new(3, 4)];
        assert_eq!(board.filled, expected);
    }

    #[test]
    fn test_clear_one_row_filled() {
        let mut board = Board::new(4, 5);
        board.filled = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0), // Row 0 is full
            Point::new(0, 1),
            Point::new(1, 2),
            Point::new(2, 2),
        ];

        board.clear_board();

        // Row 0 is cleared, and the points above it shift down
        let expected = vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 1)];
        assert_eq!(board.filled, expected);
    }

    #[test]
    fn test_clear_multiple_rows_filled() {
        let mut board = Board::new(4, 5);
        board.filled = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0), // Row 0 is full
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1), // Row 1 is full
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(2, 3),
        ];

        board.clear_board();

        // Rows 0 and 1 are cleared, and points above them shift down
        let expected = vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 1)];
        assert_eq!(board.filled, expected);
    }
}

#[cfg(test)]
mod board_block_tests {
    use super::*;

    #[test]
    fn test_block_touches_filled() {
        let mut board = Board::new(4, 5);
        board.filled = vec![Point::new(1, 3), Point::new(2, 3)];

        let block = Block::new(Point::new(1, 2), BlockShape::Line);

        // Block touches the filled points
        assert!(board.block_touches(&block));

        let non_touching_block = Block::new(Point::new(3, 0), BlockShape::Line);

        // Block does not touch any filled points
        assert!(!board.block_touches(&non_touching_block));
    }

    #[test]
    fn test_block_touches_bottom() {
        let board = Board::new(4, 5);

        let block = Block::new(Point::new(0, 4), BlockShape::Square);

        // Block touches the bottom of the board
        assert!(board.block_touches(&block));
    }

    #[test]
    fn test_place_block() {
        let mut board = Board::new(4, 5);

        let block = Block::new(Point::new(0, 0), BlockShape::Square);

        board.place_block(&block);

        let mut expected_filled = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];

        board.sort_filled();
        expected_filled.sort_by_key(|p| (p.get_x(), p.get_y())); // Sort by y, then x

        assert_eq!(board.filled, expected_filled);
    }
}
