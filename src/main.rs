extern crate crossterm;

const UPDATES_PER_SECONDS: u64 = 60;
const UPDATE_SPEED: Duration = Duration::from_millis(1000 / UPDATES_PER_SECONDS);

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand, style::Print};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    start();
}

fn run() {
    let mut next_time = Instant::now();
    let running = true;
    let mut i = 0;
    while running {
        let current_time = Instant::now();
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input

            // Update
            i += 1;
            println!("hello world! tick #{}", i);

            // Render if we've updated
            if current_time < next_time {
                println!("hello world!");
            }
        } else {
            let sleep_time = next_time.duration_since(current_time);
            if sleep_time > Duration::new(0, 0) {
                sleep(sleep_time);
            }
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
