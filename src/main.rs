extern crate crossterm;

use crossterm::{cursor, style::Print, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod input;

const TPS: u64 = 20;
const TICK_TIME: Duration = Duration::from_millis(1000 / TPS);

fn main() {
    let mut stdout = stdout();

    // Set up terminal
    stdout.queue(terminal::EnterAlternateScreen).unwrap();
    stdout.queue(cursor::Hide).unwrap();
    stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout.flush().unwrap();

    let input_receiver = input::get_input();

    let mut last_time = Instant::now();

    let mut update_count = 0;
    let mut render_count = 0;
    let mut sleep_time = Duration::from_millis(0);

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);

        last_time = current_time;

        // Handle input
        while let Ok(char) = input_receiver.try_recv() {
            match char {
                'q' => break 'running,
                _ => (),
            }
        }
        // Update
        update_count += 1;

        // Render
        stdout.queue(cursor::MoveTo(0, 0)).unwrap()
              .queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap()
              .queue(Print(format!("Updates: {:?} Renders: {:?} Delta Time: {:?} Last Sleep Time: {:?}", update_count, render_count, delta_time, sleep_time))).unwrap();

        stdout.flush().unwrap();

        render_count += 1;

        if delta_time < TICK_TIME {
            sleep_time = TICK_TIME - delta_time;
            sleep(sleep_time);
        }
    }

    // Restore terminal after game is finished
    terminal::disable_raw_mode().unwrap();
    stdout.queue(cursor::Show).unwrap();
    stdout.queue(terminal::LeaveAlternateScreen).unwrap();
    stdout.flush().unwrap();
    println!("Game exited successfully");
}
