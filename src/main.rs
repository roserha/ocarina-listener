use cpal::{
    traits::{DeviceTrait, HostTrait},
    StreamConfig,
}; // Microphone Crate

use console::{Style, Term};  // Cute Console Crate

mod notes;

mod songs;

use std::collections::{VecDeque, BTreeMap};


fn main() {
    println!("Hello, {}! :)"/*\nHope {} likes this uwu"*/
    , Style::new().color256(213).bold().apply_to("World")/*
    , Style::new().color256(135).bold().apply_to("someone")*/);

    // Create terminal interface
    let term = Term::stdout();

    // Create and load songbook
    println!("Loading songs.."); term.move_cursor_up(1).unwrap();
    let mut songbook:BTreeMap<String, String> = BTreeMap::new();
    songs::load_songbook(&mut songbook);
    println!("Loaded {} songs.",
    Style::new().cyan().bold().apply_to(songbook.len()));

    // Get Microphone Info
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No devices found :(");
    let sconfig:StreamConfig = device.default_input_config().unwrap().into();
    println!("Chosen device: \"{}\"\n", device.name().unwrap());

    // Print FSN skeleton
    println!("Frequency {}\nSemitones {}\nNote {}\n", 
    Style::new().green().bold().apply_to("--"),
    Style::new().red().bold().apply_to("--"),
    Style::new().blue().bold().apply_to("--"));

    // Create note-keepers
    let mut initial_notes: Vec<i16> = Vec::new();
    let mut last_note: String = String::from("--");
    let mut played_notes: VecDeque<String> = VecDeque::new();
    let sample_rate = sconfig.sample_rate.0;

    // Run Mic Data
    let _stream = device.build_input_stream(
        &sconfig,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            notes::get_pitch(&data, &mut played_notes, &term, &mut initial_notes, &mut last_note, sample_rate);

            if initial_notes.len() == 0 {
                songs::check_played_notes(&mut played_notes, &mut songbook);
            }
        },
        move |err| {
            print!("not success :( -> {}", err);
        }, 
        None
    );

    loop { };

}