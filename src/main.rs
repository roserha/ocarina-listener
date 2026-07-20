use cpal::{
    StreamConfig,
    traits::{DeviceTrait, HostTrait},
}; // Microphone Crate

use console::{Style, Term}; // Cute Console Crate

mod notes;
mod songs;

use std::collections::{BTreeMap, VecDeque};

use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Hello, {}! :)", /*\nHope {} likes this uwu"*/
        Style::new().color256(213).bold().apply_to("World")  /*
                         , Style::new().color256(135).bold().apply_to("someone")*/
    );

    // Create terminal interface
    let term = Term::stdout();

    // Create and load songbook
    println!("Loading songs..");
    term.move_cursor_up(1).unwrap();
    let mut songbook: BTreeMap<String, String> = BTreeMap::new();
    songs::load_songbook(&mut songbook);
    println!(
        "Loaded {} songs.",
        Style::new().cyan().bold().apply_to(songbook.len())
    );

    // Get Microphone Info
    let host = cpal::default_host();
    let device = host
        .input_devices()
        .expect("No devices found :(")
        .find(|d| {
            d.name()
                .map(|n| n.contains("googlevoicehat") || n.contains("sndrpigooglevoi"))
                .unwrap_or(false)
        })
        .expect("voicehat mic not found :((");

    let sconfig: StreamConfig = device
        .default_input_config()
        .expect("failed to get inptu configs :(((")
        .into();

    println!("Chosen device: \"{}\"\n", device.name().unwrap());

    // Print FSN skeleton
    println!(
        "Frequency {}\nSemitones {}\nNote {}\n",
        Style::new().green().bold().apply_to("--"),
        Style::new().red().bold().apply_to("--"),
        Style::new().blue().bold().apply_to("--")
    );

    // Create note-keepers
    let mut initial_notes: Vec<i16> = Vec::new();
    let mut last_note: String = String::from("--");
    let mut played_notes: VecDeque<String> = VecDeque::new();
    let sample_rate = sconfig.sample_rate.0;

    // Set up 16-segment display MPSC channel to not throttle audio thread
    let (tx, rx) = mpsc::channel::<notes::DisplayData>();
    let mut sender = loop {
        match UnixStream::connect("/tmp/ocarina-listener.sock") {
            Ok(stream) => break stream,
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    };

    drop(term);

    std::thread::spawn(move || {
        let term = Term::stdout();

        while let Ok(noteinfo) = rx.recv() {
            // Reset terminal cursor position

            for _ in 0..4 {
                term.move_cursor_up(1).unwrap();
                term.clear_line().unwrap();
            }

            // Print data on console

            println!(
                "Frequency {}\nSemitones {}\nNote {}",
                Style::new().green().bold().apply_to(noteinfo.freq),
                Style::new().red().bold().apply_to(noteinfo.semitones),
                Style::new()
                    .blue()
                    .bold()
                    .apply_to(noteinfo.note_name.clone())
            );

            for note in noteinfo.notes_played.iter() {
                print!("{} ", Style::new().yellow().bold().apply_to(note));
            }

            println!();

            // Send all the data via UNIX Stream to GUI
            let mut noteinfo_playednotes = String::new();

            for note in noteinfo.notes_played.iter().rev().take(4).rev() {
                noteinfo_playednotes.push_str(format!("{} ", note).as_str());
            }

            if !noteinfo_playednotes.is_empty() {
                noteinfo_playednotes.pop();
            }

            sender
                .write_all(
                    format!(
                        "{}||{}||{}||{}\n",
                        noteinfo.freq, noteinfo.semitones, noteinfo.note_name, noteinfo_playednotes
                    )
                    .as_bytes(),
                )
                .unwrap();
            sender.flush().unwrap();
        }
    });

    // Run Mic Data
    let _stream = device.build_input_stream(
        &sconfig,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            notes::get_pitch(
                data,
                &mut played_notes,
                &tx,
                &mut initial_notes,
                &mut last_note,
                sample_rate,
            );

            if initial_notes.is_empty() {
                songs::check_played_notes(&mut played_notes, &mut songbook);
            }
        },
        move |err| {
            print!("not success :( -> {}", err);
        },
        None,
    );

    loop {
        thread::park();
    }
}
