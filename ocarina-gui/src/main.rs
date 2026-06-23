// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::ptr::read;
use std::sync::mpsc;
use std::os::unix::net::{UnixStream,UnixListener};
use std::io::{BufRead, BufReader};
use slint::{Model, ModelRc, VecModel};
use std::rc::Rc;

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
    let _ = std::fs::remove_file("/tmp/ocarina-listener.sock");
    let listener = UnixListener::bind("/tmp/ocarina-listener.sock").unwrap();

    std::thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            let stream_reader = BufReader::new(stream);
            
            for stream_line in stream_reader.lines() {
                match stream_line {
                    Ok(line) => {tx.send(line).ok();}
                    Err(_) => {}
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
                    let raw_msgs = msg.split("||").collect::<Vec<&str>>();

                    
                    let played_notes:Vec<slint::SharedString> = raw_msgs[3].split(" ").map(|s| s.trim().into()).collect();
                    
                    let played_notes_rc = Rc::new(VecModel::from(played_notes));
                    
                    ui.set_playedNotes(ModelRc::new(played_notes_rc.clone()));

                    ui.set_listening(raw_msgs[2] != "--");

                    ui.set_currentlyPlayingSong(raw_msgs[2].into());
                }
            }
        },
    );

    ui.run()?;

    Ok(())
}
