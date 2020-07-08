extern crate crossterm;

use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    setup_terminal();
    run(10);
    restore_terminal();
}

fn run(tps: u64) {
    let tick_time = 1000 / tps;
    let start_time = Instant::now();
    let mut next_time: u64 = start_time.elapsed().as_millis() as u64;

    let mut update_count = 0;
    let mut render_count = 0;

    let running = true;
    while running {
        let current_time: u64 = start_time.elapsed().as_millis() as u64;
        if current_time >= next_time {
            next_time += tick_time;
            // Handle input

            // Update
            update_count += 1;

            // Render
            if current_time < next_time {
                render_count += 1;
                stdout().queue(cursor::MoveTo(0, 0)).unwrap().queue(Print(format!("Updates: {}", update_count))).unwrap();
                stdout().queue(cursor::MoveTo(0, 1)).unwrap().execute(Print(format!("Renders: {}", render_count))).unwrap();
            }
        } else {
            sleep(Duration::from_millis(next_time - current_time));
        }
    }
}

fn setup_terminal() {
    // Set up terminal
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn restore_terminal() {
    // Restore terminal after game is finished
    stdout().execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited successfully");
}
