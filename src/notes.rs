use pitch_detection::{
    detector::{
        PitchDetector, mcleod::McLeodDetector,
    },
    Pitch,
};



use std::cell::RefCell;
use std::collections::{VecDeque, BTreeMap};

use std::sync::mpsc::Sender;

// Display thread data
pub struct DisplayData {
    pub freq: f32,
    pub semitones: i16,
    pub note_name: String,
    pub notes_played: Vec<String>,
}

// the audio callback always runs on its own dedicated thread, so thread_local is
// the right home for the detector -- safe, no unsafe, no Send shenanigans
thread_local! {
    static DETECTOR: RefCell<McLeodDetector<f32>> = RefCell::new(McLeodDetector::new(512, 256));
}

const NOTES: &'static [&'static str] = &["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

pub fn interpret_note(initial_notes: &mut Vec<i16>, notes_played: &mut VecDeque<String>, last_note: &mut String) {
    // Only interpret note if we have 30 notes in our backlog

    if initial_notes.len() >= 30 {
        // Create hash tables for mode calculation
        // (keys are the tones/octaves, values are frequency)
        let mut tones_captured: BTreeMap<i16, i8> = BTreeMap::new();
        let mut octaves_captured: BTreeMap<i16, i8> = BTreeMap::new();
        let mut num_notes_captured = 0;

        // For each note captured (that isn't an invalid note),
        // capture its tone and octave and add it to hash table

        for note in initial_notes.iter() {
            if *note != i16::MIN {
                let tone = note.rem_euclid(12);
                let octave = 1.max(note.div_euclid(12) + 4);

                match tones_captured.get(&tone) {
                    Some(&num) => {tones_captured.insert(tone, &num + 1);}
                    None => {tones_captured.insert(tone, 1);}
                }

                match octaves_captured.get(&octave) {
                    Some(&num) => {octaves_captured.insert(octave, &num + 1);}
                    None => {octaves_captured.insert(octave, 1);}
                }

                num_notes_captured += 1;
            }
        }

        // Clear initial notes backlog

        initial_notes.clear();

        let mut note_played = String::from("--");

        // If we have more notes than silence,
        // get mode of tone and octave
        // and get the note name

        if num_notes_captured > 15 {
            let mut played_tone: i16 = 0; let mut num_tone_detected = 0;
            let mut played_octave: i16 = 0; let mut num_octave_detected = 0;

            for (k, v) in tones_captured {
                if v >= num_tone_detected {
                    played_tone = k;
                    num_tone_detected = v;
                }
            }

            for (k, v) in octaves_captured {
                if v >= num_octave_detected {
                    played_octave = k;
                    num_octave_detected = v;
                }
            }

            let played_note_semitone:i16 = played_tone + (played_octave - 4) * 12;

            note_played = NOTES[(played_note_semitone.rem_euclid(12)) as usize].to_owned();
            note_played.push_str(&(((played_note_semitone / 12) + 4).to_string().to_owned()));
        }

        // If note name is different from last note, this is a new note!
        // Or it might be a new silence

        if note_played != *last_note {
            *last_note = note_played.clone();

            // If this new note wasn't silence, let's add it to our queue
            if note_played != "--" {
                notes_played.push_back(note_played.clone());

                if notes_played.len() >= 12 {
                    notes_played.pop_front();
                }
            }
        }
    }
}

pub fn get_pitch(data: &[f32], notes_played: &mut VecDeque<String>, tx: &Sender<DisplayData>, initial_notes: &mut Vec<i16>, last_note: &mut String, sample_rate: u32) {
    // Let's calculate the volume via Root Mean Square
    let mut rms:f32 = 0.0;

    if data.len() < 512 { return; }

    // LOOP UNROLLING BABYYYYY
    for i in 0..64 {
        rms += data[8*i] * data[8*i];
        rms += data[8*i+1] * data[8*i+1];
        rms += data[8*i+2] * data[8*i+2];
        rms += data[8*i+3] * data[8*i+3];
        rms += data[8*i+4] * data[8*i+4];
        rms += data[8*i+5] * data[8*i+5];
        rms += data[8*i+6] * data[8*i+6];
        rms += data[8*i+7] * data[8*i+7];
    }

    rms = (rms / 512.0).sqrt();

    // Now let's get the frequency
    // this used to use a static mut + unsafe block because the detector lived in this
    // secondary thread and i had no clue how to do it otherwise. sooooooo, my bad...
    // fixed it with thread_local! + RefCell -- safe, and still lives on the audio thread (pls laugh)

    let freq:Option<Pitch<f32>> = DETECTOR.with(|det| {
        det.borrow_mut().get_pitch(&data[..512], sample_rate as usize, 0.0, 0.6)
    });

    let actual_freq: f32 = if rms < 0.01 {
        0.0001
    } else {
        match freq {
            Some(f) => f.frequency,
            None => {0.0001}
        }
    };

    // With the frequency, let's calculate the relative semitone from C4
    // (Explanation: note frequencies can be looked at via 440x2^(semitones_above_A4/12),
    //  so we can just do fun maths to get the original semitones above A4, and in turn C4)

    let mut semitones = (12.0*(actual_freq/440.0).log2()).round() as i16 + 9;

    // Get note name

    let mut note_name = NOTES[(semitones.rem_euclid(12)) as usize].to_owned();
    note_name.push_str(&(((semitones / 12) + 4).to_string()));

    // If it's an invalid note, drop note name

    if actual_freq < 0.01 {
        note_name = String::from("--");
        semitones = i16::MIN;
    }

    // Now let's add the note and interpret it!

    initial_notes.push(semitones);
    interpret_note(initial_notes, notes_played, last_note);

    // Finally, let's send everything to the display thread using note_info
    // 0: {actual_freq}
    // 1: {semitones}
    // 2: {note_name}
    // 3: notes_played
    tx.send(DisplayData {
        freq: actual_freq,
        semitones,
        note_name,
        notes_played: notes_played.iter().cloned().collect(),
    }).ok();
}
