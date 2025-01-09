use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Stylize},
    terminal::{self, ClearType},
};
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

mod point;
mod block;
mod board;

use block::{Block, BlockShape};
use board::Board;
use point::Point;

fn draw_board(board: &Board, current_block: &Block) {
    let mut stdout = stdout();

    // Clear the terminal
    execute!(stdout, terminal::Clear(ClearType::All)).unwrap();

    // Draw the board boundary
    for y in 0..board.y_dim {
        for x in 0..board.x_dim {
            let is_filled = board
                .filled
                .iter()
                .any(|p| p.get_x() == x && p.get_y() == y);
            let is_block = current_block
                .coordinates
                .iter()
                .any(|p| p.get_x() == x && p.get_y() == y);

            if is_filled || is_block {
                print!("{}", "[]".with(Color::Blue));
            } else {
                print!("  ");
            }
        }
        println!(); // Newline after each row
    }

    stdout.flush().unwrap();
}
fn main() {
    // Initialize terminal
    terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    // Get terminal size
    let (term_width, term_height) = terminal::size().unwrap();

    // Adjust board size to fit the terminal
    let x_dim = (term_width / 2).min(20); // Each cell is "[]" = 2 characters wide
    let y_dim = term_height.min(20);

    // Create a new board with dimensions fitting the terminal
    let mut board = Board::new(x_dim as usize, y_dim as usize);
    let mut current_block = Block::new(Point::new((x_dim / 2) as i32 - 1, 0), BlockShape::T);



    let mut last_tick = Instant::now();

    loop {
        // Check for user input
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Left => {
                        let moved_block = Block {
                            coordinates: current_block
                                .coordinates
                                .iter()
                                .map(|p| p.add(&Point::new(-1, 0)))
                                .collect(),
                            ..current_block.clone()
                        };
                        if moved_block.coordinates.iter().all(|p| p.get_x() >= 0 && p.get_x() < board.x_dim) {
                            current_block = moved_block;
                        }
                    }
                    KeyCode::Right => {
                        let moved_block = Block {
                            coordinates: current_block
                                .coordinates
                                .iter()
                                .map(|p| p.add(&Point::new(1, 0)))
                                .collect(),
                            ..current_block.clone()
                        };
                        if moved_block.coordinates.iter().all(|p| p.get_x() >= 0 && p.get_x() < board.x_dim) {
                            current_block = moved_block;
                        }
                    }
                    
                    KeyCode::Up => {
                        // Rotate block
                        if let Ok(rotated_block) = current_block.rotate() {
                            if !board.block_touches(&rotated_block) {
                                current_block = rotated_block;
                            }
                        }
                    }
                    KeyCode::Down => {
                        // Drop block faster
                        if let Ok(dropped_block) = current_block.push_down() {
                            if !board.block_touches(&dropped_block) {
                                current_block = dropped_block;
                            }
                        }
                    }
                    KeyCode::Char('q') => break, // Quit game
                    _ => {}
                }
            }
        }

        // Auto drop the block every 500ms
        if last_tick.elapsed() >= Duration::from_millis(500) {
            last_tick = Instant::now();
            if let Ok(dropped_block) = current_block.push_down() {
                if board.block_touches(&dropped_block) || dropped_block.coordinates.iter().any(|p| p.get_y() >= board.y_dim) {
                    board.place_block(&current_block);
                    current_block = Block::new(Point::new((x_dim / 2) as i32 - 1, 0), BlockShape::T);
                    board.clear_board();
                } else {
                    current_block = dropped_block;
                }
            }
             else {
                board.place_block(&current_block);
                current_block = Block::new(Point::new((x_dim / 2 - 2) as i32, 0), BlockShape::T);
                board.clear_board();
            }
        }

        // Render the board
        draw_board(&board, &current_block);
    }

    // Cleanup terminal
    execute!(stdout, cursor::Show).unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
