extern crate crossterm;

use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::{Stdout, Write, stdout};

mod GameLoop;

const MS_PER_UPDATE: f64 = 1000.0 / 60.0;

fn main() {
    let mut stdout = stdout();
    setup_terminal(&mut stdout);

    let mut timestep = GameLoop::TimeStep::new();
    let mut lag = 0.0;

    let running = true;

    'gameloop: loop {
        if !running {
            break 'gameloop;
        }
        // Handle input

        let delta = timestep.delta();
        lag += delta;

        while lag >= MS_PER_UPDATE {
            // Update
            lag -= MS_PER_UPDATE;
        }

        // Render
        stdout.queue(cursor::MoveTo(0, 0))
              .unwrap()
              .queue(terminal::Clear(terminal::ClearType::CurrentLine))
              .unwrap()
              .queue(Print(format!("Lag: {:?} Delta: {:?} FPS: {:?}", lag, delta, timestep.frame_rate().unwrap_or(0))))
              .unwrap();

        stdout.flush().unwrap();
    }

    restore_terminal(&mut stdout);
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
