// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::ptr::read;
use std::sync::mpsc;
use std::os::unix::net::{UnixStream,UnixListener};
use std::io::{BufRead, BufReader};

pub struct DisplayData {
    pub freq: f32,
    pub semitones: i16,
    pub note_name: String,
    pub notes_played: Vec<String>,
}

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    
    // Set up display info MPSC channel to run parallel with UI thread
    let (tx, rx) = mpsc::channel::<String>();
    let listener = UnixListener::bind("/tmp/ocarina-listener.sock").unwrap();

    std::thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream_content) => {
                    let read_stream = BufReader::new(stream_content).lines();
                    let mut complete_stream = String::new();

                    for stream_line in read_stream {
                        match stream_line {
                            Ok(line) => complete_stream.push_str(&line.to_string()),
                            Err(_) => {}
                        }
                    }

                    tx.send(complete_stream).ok();
                }
                Err(err) => {
                    print!("Error in MPSC GUI Channel: {}", err);
                }
            }
        }
    });

    ui.on_request_increase_value({
        let ui_handle_internal = ui.as_weak();
        move || {
            let ui = ui_handle_internal.unwrap();
            ui.set_currentlyPlayingSong(slint::SharedString::from("helllo world"));
        }
    });

    let ui_loop = slint::Timer::default();

    ui_loop.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(16),
        move || {
            if let Ok(msg) = rx.try_recv() {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_currentlyPlayingSong(msg.into());
                }
            }
        },
    );

    ui.run()?;

    Ok(())
}
