extern crate crossterm;

use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::{Stdout, Write, stdout};
use std::thread::sleep;

use std::time::{Duration, Instant};

const TPS: u64 = 10;
const TICK_TIME: Duration = Duration::from_millis(1000 / TPS);

fn main() {
    let mut stdout = stdout();
    setup_terminal(&mut stdout);

    let mut last_time = Instant::now();

    loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);

        last_time = current_time;

        // Handle input
        // Update
        // Render

        if delta_time < TICK_TIME {
            sleep(TICK_TIME - delta_time);
        }
    }

}

fn setup_terminal(stdout: &mut Stdout) {
    // Set up terminal
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn restore_terminal(stdout: &mut Stdout) {
    // Restore terminal after game is finished
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited successfully");
}
