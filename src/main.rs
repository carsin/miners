extern crate crossterm;

use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::thread::sleep;

use std::time::{Duration, Instant};

const TPS: u64 = 20;
const TICK_TIME: Duration = Duration::from_millis(1000 / TPS);

fn main() {
    let mut stdout = stdout();
    setup_terminal(&mut stdout);

    let mut last_time = Instant::now();

    let mut update_count = 0;
    let mut render_count = 0;
    let mut sleep_time = Duration::from_millis(0);

    loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);

        last_time = current_time;

        // Handle input
        // Update
        update_count += 1;
        // Render
        stdout
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap()
            .queue(Print(format!("Updates: {:?} Renders: {:?} Delta Time: {:?} Last Sleep Time: {:?}",update_count, render_count, delta_time, sleep_time))).unwrap();
        stdout.flush().unwrap();

        render_count += 1;

        if delta_time < TICK_TIME {
            sleep_time = TICK_TIME - delta_time;
            sleep(sleep_time);
        }
    }

    restore_terminal(&mut stdout);
}

fn setup_terminal(stdout: &mut Stdout) {
    // Set up terminal
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}

fn restore_terminal(stdout: &mut Stdout) {
    // Restore terminal after game is finished
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited successfully");
}
