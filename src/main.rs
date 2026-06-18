use cpal::{
    traits::{DeviceTrait, HostTrait},
    StreamConfig,
}; // Microphone Crate

use console::{Style, Term};  // Cute Console Crate

mod notes;
mod songs;

use std::collections::{VecDeque, BTreeMap};

use rppal::{
    i2c::I2c,
    hal::Delay};

use std::sync::mpsc;


fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Set up 16-segment display MPSC channel to not throttle audio thread
    let (tx, rx) = mpsc::channel::<notes::DisplayData>();

    drop(term);

    std::thread::spawn(move || {
        let term = Term::stdout();
        let mut lcd_i2c = I2c::new().expect("Error creating I2c!!"); 

        let mut delay = Delay::new();

        let mut lcd = match lcd_lcm1602_i2c::sync_lcd::Lcd::new(&mut lcd_i2c, &mut delay)
            .with_address(0x27)
            .with_cursor_on(false)
            .with_rows(2)
            .init() {
                Ok(l) => l,
                Err(_) => panic!("Error creating LCD!!")
        };

        lcd.clear().unwrap();
        lcd.set_cursor(1,0).unwrap();
        lcd.write_str("Ocarina Listener").unwrap();

        let mut last_notes_played: Vec<String> = Vec::new();

        while let Ok(noteinfo) = rx.recv() {
            // Reset terminal cursor position
        
            for _ in 0..4 {
                term.move_cursor_up(1).unwrap();
                term.clear_line().unwrap();
            }
        
            // Print data on console
        
            println!("Frequency {}\nSemitones {}\nNote {}",
            Style::new().green().bold().apply_to(noteinfo.freq),
            Style::new().red().bold().apply_to(noteinfo.semitones),
            Style::new().blue().bold().apply_to(noteinfo.note_name));
        
            for note in noteinfo.notes_played.iter(){
                print!("{} ", Style::new().yellow().bold().apply_to(note));
            }
        
            print!("\n");

            // Print data on LCD display

            if last_notes_played != noteinfo.notes_played {
                lcd.set_cursor(0,0).unwrap();
            
                for note in noteinfo.notes_played.iter().rev().take(4).rev(){
                    lcd.write_str(format!("{:^4}", note).as_str()).unwrap();
                }
            
                lcd.set_cursor(1,0).unwrap();
                lcd.write_str(format!("Ocarina{}Listener", if noteinfo.freq < 0.01 { " " } else { "*" }).as_str()).unwrap();
            }

            last_notes_played = noteinfo.notes_played.clone();
        
 
        }
    });

    

    // Run Mic Data
    let _stream = device.build_input_stream(
        &sconfig,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            notes::get_pitch(&data, &mut played_notes, &tx, &mut initial_notes, &mut last_note, sample_rate);

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
