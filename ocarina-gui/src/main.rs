// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ModelRc, VecModel};
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::process::{Command, Child};
use std::rc::Rc;
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub struct DisplayData {
    pub freq: f32,
    pub semitones: i16,
    pub note_name: String,
    pub notes_played: Vec<String>,
}

// Audio settings
pub struct AudioPrefs {
    pub backlog_size: u8,
    pub silence_threshold: u8,
    pub sensitivity: f32,
}

slint::include_modules!();

use syslog::{Facility, Formatter3164};

// debug using `tail -f /var/log/messages | grep "ocarina-"`

fn init_logging() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_DAEMON,
        hostname: None,
        process: "ocarina-gui".into(),
        pid: std::process::id(),
    };

    match syslog::unix(formatter) {
        Err(e) => eprintln!("could not connect to syslog: {e}"),
        Ok(writer) => {
            let _ = log::set_boxed_logger(Box::new(syslog::BasicLogger::new(writer)))
                .map(|()| log::set_max_level(log::LevelFilter::Info));
        }
    }

    std::panic::set_hook(Box::new(|info| {
        log::error!("PANIC: {info}");
    }));
}

fn update_ip() -> String {
    match Command::new("sh")
        .arg("-c")
        .arg(r#"ip addr show wlan0 | grep "inet " | awk '{print $2}' | cut -d/ -f1"#)
        .output()
    {
        Ok(out) => String::from_utf8(out.stdout)
            .unwrap_or_default()
            .trim()
            .to_string(),
        Err(_) => String::new(),
    }
}

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_HASH"), ")");

fn binary_version(path: &str) -> String {
    std::process::Command::new(path)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into())
}

fn os_version() -> String {
    std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VERSION="))
                .map(|l| l.trim_start_matches("VERSION=").trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "unknown".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        println!("{VERSION}");
        return Ok(());
    }
    init_logging();
    log::info!("ocarina-gui starting (version {VERSION})");
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
                if let Ok(line) = stream_line {
                    tx.send(line).ok();
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

    let mut last_ip_calculation = Instant::now() - Duration::from_millis(1500);
    let mut first_ip_calculation = last_ip_calculation;
    let mut ip_addy = String::new();
    let mut aplay_tasks: Vec<Child> = vec![];

    let bg_audio = Command::new("sh")
    .arg("-c")
    .arg(format!("aplay -D hw:CARD=b1,DEV=0 -t raw -f S16_LE -r 48000 -c 2 /dev/zero"))
    .spawn()
    .expect("Failed to start audio process");

    aplay_tasks.push(bg_audio);

    ui_loop.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(16),
        move || {
            if let Ok(msg) = rx.try_recv() {
                if let Some(ui) = ui_handle.upgrade() {
                    let raw_msgs = msg.split("||").collect::<Vec<&str>>();

                    let played_notes: Vec<slint::SharedString> =
                        raw_msgs[3].split("%").map(|s| s.trim().into()).collect();

                    let played_notes_rc = Rc::new(VecModel::from(played_notes.clone()));

                    if played_notes[0].len() > 3 {
                        let (cue_raw, song) = played_notes[0].split_at(1);

                        let cue = match cue_raw {
                            "o" => "song_correct",
                            _ => "secret_found"
                        };

                        let sp_ss = ui.get_songPlaying();
                        let sp = sp_ss.as_str();

                        if song != sp {
                            ui.set_songPlaying(slint::SharedString::from(song));
                            ui.invoke_show_song();
    
                            if aplay_tasks.len() == 1 {
                                let aplay = match Command::new("sh")
                                    .arg("-c")
                                    .arg(format!("aplay -D plughw:CARD=b1,DEV=0 /usr/share/ocarina/sounds/{cue}.wav && aplay -D plughw:CARD=b1,DEV=0 \"/usr/share/ocarina/sounds/{song}.wav\""))
                                    .spawn()
                                    {
                                        Ok(c) => c,
                                        Err(e) => {
                                            log::error!("error when creating aplay: {}", e);
                                            panic!();
                                        }
                                    };
        
                                aplay_tasks.push(aplay);
                            }
                        } else if aplay_tasks.len() == 2 {
                            match aplay_tasks[1].try_wait() {
                                // 1. Still Running
                                Ok(None) => { }
                                // 2. Finished Successfully (or with a specific code)
                                Ok(Some(status)) if status.success() => {
                                    aplay_tasks.pop();
                                    ui.invoke_hide_song();
                                }
                                // 3. Finished but Failed
                                Ok(Some(status)) => {
                                    // It stopped, but it failed (e.g., sound card missing, wrong path).
                                    ui.set_songPlaying(slint::SharedString::from(format!("Err - {:?}", status.code())));
                                }
                                // Error querying the OS
                                Err(e) => ui.set_songPlaying(slint::SharedString::from(format!("OSErr - {}", e))),
                            }
                        }
                        

                    }

                    ui.set_playedNotes(ModelRc::new(played_notes_rc.clone()));

                    ui.set_listening(raw_msgs[2] != "--");

                    ui.set_currentlyPlayingSong(raw_msgs[2].into());

                    if last_ip_calculation.elapsed() >= Duration::from_millis(1500) {
                        ip_addy = update_ip();
                        ui.set_ipAddy(slint::SharedString::from(ip_addy.clone()));

                        if first_ip_calculation == last_ip_calculation
                        {
                            ui.set_OcarinaGUIVersion(VERSION.into());
                            ui.set_OcarinaListenerVersion(binary_version("/usr/bin/ocarina-listener").into());
                            ui.set_OcarinaOSVersion(os_version().into());
                        }

                        last_ip_calculation = Instant::now();
                    }
                }
            }
        },
    );

    ui.run()?;

    Ok(())
}
