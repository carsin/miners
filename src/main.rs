extern crate crossterm;

const TICKS_PER_SECOND: u64 = 1;
const TICKS_TO_SKIP: u64 = 1000000000 / TICKS_PER_SECOND;
const MAX_FRAMESKIP: i64 = 10; // Max number of frames before rendering. Game rendering to drop to this speed before updating slows.

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, style::Print};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    start();
}

fn run() {
    let start_time = Instant::now();
    let mut next_time: u64 = start_time.elapsed().as_nanos() as u64;
    let max_time_diff: u64 = 500;

    let mut skipped_frames = 1;
    let max_skipped_frames = 5;

    let mut update_count = 0;
    let mut render_count = 0;

    let running = true;
    while running {
        let mut curr_time: u64 = start_time.elapsed().as_nanos() as u64;

        if (curr_time - next_time) > max_time_diff {
            next_time = curr_time;
        }

        if curr_time >= next_time {
            next_time += TICKS_TO_SKIP;
            // Update
            update_count += 1;

            if curr_time < next_time || skipped_frames > max_skipped_frames {
                // Render
                render_count += 1;
                stdout().queue(cursor::MoveTo(0, 0)).unwrap()
                        .queue(Print(format!("Updates: {}", update_count))).unwrap();
                stdout().queue(cursor::MoveTo(0, 1)).unwrap()
                        .queue(Print(format!("Renders: {}", render_count))).unwrap();

                skipped_frames = 1
            } else {
                skipped_frames += 1;
            }
        } else {
            let sleep_time = next_time - curr_time;
            if sleep_time > 0 {
                sleep(Duration::from_nanos(sleep_time));
            }
        }
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
