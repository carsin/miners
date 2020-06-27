extern crate crossterm;

const TICKS_PER_SECOND: u64 = 1;
const TICKS_TO_SKIP: u64 = 1000 / TICKS_PER_SECOND;
const MAX_FRAMESKIP: i64 = 10; // Max number of frames before rendering. Game rendering to drop to this speed before updating slows.

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, style::Print};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    start();
}

fn run() {
    let start_time = Instant::now();
    let mut tick_count: u128 = start_time.elapsed().as_millis();
    let mut loops: i64;

    let mut update_count = 0;
    let mut render_count = 0;

    let running = true;
    while running {
        loops = 0;

        while start_time.elapsed().as_millis() > tick_count && loops < MAX_FRAMESKIP {
            // Update
            update_count += 1;

            tick_count += TICKS_TO_SKIP as u128;
            loops += 1;
        }
        // Render
        render_count += 1;
        stdout().queue(cursor::MoveTo(0, 0)).unwrap()
                .queue(Print(format!("Updates: {}", update_count))).unwrap();
        stdout().queue(cursor::MoveTo(0, 1)).unwrap()
                .queue(Print(format!("Renders: {}", render_count))).unwrap();
    }
    stop(); // Stop after loop breaks
}

fn start() {
    // Set up terminal
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    // Start game loop
    run();
}

fn stop() {
    // Restore terminal after game is finished
    stdout().execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited successfully");
}
