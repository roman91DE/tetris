mod block;
mod board;
mod point;

use crate::block::{Block, BlockShape};
use crate::board::Board;
use crate::point::Point;
use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute, terminal};
use std::io::{self};
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block as TuiBlock, Borders, Paragraph};
use tui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize game state
    let mut board = Board::new(10, 20); // 10x20 board
    let mut current_block = Block::new(Point::new(4, 0), BlockShape::get_rand()); // Start with a Line block
    let mut game_over = false;
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(500);
    let mut score = 0; // Initialize the score

    loop {
        // Draw the game state
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(80), // Board
                    Constraint::Percentage(20), // Score
                ])
                .split(f.size());

            // Draw the board
            let board_widget = draw_board(&board, &current_block);
            f.render_widget(board_widget, chunks[0]);

            // Draw the score
            let score_widget = Paragraph::new(format!("Score: {}", score))
                .block(TuiBlock::default().borders(Borders::ALL).title("Score"));
            f.render_widget(score_widget, chunks[1]);
        })?;

        // Handle user input
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => {
                        if let Some(moved_block) =
                            current_block.translate(-1, 0, board.x_dim, board.y_dim)
                        {
                            if !board.block_touches(&moved_block) {
                                current_block = moved_block;
                            }
                        }
                    }
                    KeyCode::Right => {
                        if let Some(moved_block) =
                            current_block.translate(1, 0, board.x_dim, board.y_dim)
                        {
                            if !board.block_touches(&moved_block) {
                                current_block = moved_block;
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(moved_block) =
                            current_block.translate(0, 1, board.x_dim, board.y_dim)
                        {
                            if !board.block_touches(&moved_block) {
                                current_block = moved_block;
                            }
                        }
                    }
                    KeyCode::Char('r') | KeyCode::Up => {
                        if let Ok(rotated_block) = current_block.rotate() {
                            if !board.block_touches(&rotated_block) {
                                current_block = rotated_block;
                            }
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Auto-drop block
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if let Some(moved_block) = current_block.translate(0, 1, board.x_dim, board.y_dim) {
                if board.block_touches(&moved_block) {
                    board.place_block(&current_block);
                    score += board.clear_board(); // Update score based on cleared rows
                    current_block = Block::new(Point::new(4, 0), BlockShape::get_rand());
                    if board.block_touches(&current_block) {
                        game_over = true;
                    }
                } else {
                    current_block = moved_block;
                }
            } else {
                board.place_block(&current_block);
                score += board.clear_board(); // Update score based on cleared rows
                current_block = Block::new(Point::new(4, 0), BlockShape::get_rand());
                if board.block_touches(&current_block) {
                    game_over = true;
                }
            }
        }

        if game_over {
            terminal.draw(|f| {
                let size = f.size();
                let game_over_widget = Paragraph::new("Game Over! Press 'Q' to quit.")
                    .block(TuiBlock::default().borders(Borders::ALL).title("Game Over"));
                f.render_widget(game_over_widget, size);
            })?;
            loop {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                        break;
                    }
                }
            }
            break;
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn draw_board<'a>(board: &Board, current_block: &Block) -> Paragraph<'a> {
    let mut grid = vec![vec!['.'; board.x_dim as usize]; board.y_dim as usize];

    // Mark filled points
    for point in &board.filled {
        if point.get_x() >= 0 && point.get_y() >= 0 {
            grid[point.get_y() as usize][point.get_x() as usize] = '#';
        }
    }

    // Mark current block
    for point in &current_block.coordinates {
        if point.get_x() >= 0 && point.get_y() >= 0 {
            grid[point.get_y() as usize][point.get_x() as usize] = '*';
        }
    }

    // Render the board into a string
    let board_string = grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    Paragraph::new(board_string).block(TuiBlock::default().borders(Borders::ALL).title("Board"))
}

