use crate::point::Point;

struct Board {
    x_dim: i32,
    y_dim: i32,
    filled: Vec<Point>,
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
        self.filled.retain(|point| !completed_rows.contains(&point.get_y()));

        // Shift points above cleared rows downward
        for point in &mut self.filled {
            let rows_above = completed_rows.iter().filter(|&&row| row < point.get_y()).count() as i32;
            *point = Point::new(point.get_x(), point.get_y() - rows_above);
        }
    }
    }
    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_no_rows_filled() {
        let mut board = Board::new(4, 5);
        board.filled = vec![
            Point::new(0, 0),
            Point::new(1, 2),
            Point::new(3, 4),
        ];

        board.clear_board();

        // No rows were cleared, so the filled points remain unchanged
        let expected = vec![
            Point::new(0, 0),
            Point::new(1, 2),
            Point::new(3, 4),
        ];
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
        let expected = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(2, 1),
        ];
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
        let expected = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 1),
        ];
        assert_eq!(board.filled, expected);
    }
}
