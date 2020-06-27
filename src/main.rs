extern crate crossterm;

const UPDATES_PER_SECONDS: u128 = 10;
const UPDATE_SPEED: u128 = 1000 / UPDATES_PER_SECONDS;

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, style::Print};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Instant, Duration};

fn main() {
    start();
}

fn run() {
    let start_time = Instant::now();
    let mut next_time: u128 = start_time.elapsed().as_millis();

    let mut update_count = 0;
    let mut render_count = 0;

    let running = true;
    while running {
        let current_time: u128 = start_time.elapsed().as_millis();
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input

            // Update
            update_count += 1;

            // Render
            if current_time < next_time {
                render_count += 1;
                stdout().queue(cursor::MoveTo(0, 0)).unwrap()
                        .queue(Print(format!("Updates: {}", update_count))).unwrap();
                stdout().queue(cursor::MoveTo(0, 1)).unwrap()
                        .execute(Print(format!("Renders: {}", render_count))).unwrap();
            }
        } else {
            sleep(Duration::from_millis((next_time - current_time) as u64));
        }
    }
    stop();
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
