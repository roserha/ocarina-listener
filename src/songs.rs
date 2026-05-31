use std::collections::{VecDeque, BTreeMap};

// Load songbook.json into songbook hashmap
pub fn load_songbook(songbook: &mut BTreeMap<String, String>) {
    // Load raw json (loaded on compilation because no_std!)
    let songbook_json_contents = include_str!("songbook.json"); 
    let songbook_json = json::parse(&songbook_json_contents).unwrap();

    for (k, v) in songbook_json.entries() {
        songbook.insert(k.to_owned(), v.to_string());
    }

}

// Check to see if played notes match a song in the book
pub fn check_played_notes(played_notes: &mut VecDeque<String>, songbook: &mut BTreeMap<String, String>) {
    let mut stringified_played_notes = String::new();

    
    // Add regular octave of notes
    for note in played_notes.iter(){
        stringified_played_notes.push_str(note);
        stringified_played_notes.push_str(" ");
    }

    // Add higher octave of notes
    stringified_played_notes.push_str("-|-");
    for note in played_notes.iter(){
        let mut note_base = note.clone();
        let mut note_octave_str = note_base.split_off(note.len() - 1);
        let note_octave = note_octave_str.parse::<i16>().unwrap_or(0) + 1;
        note_octave_str = note_octave.to_string();

        stringified_played_notes.push_str(&note_base);
        stringified_played_notes.push_str(&note_octave_str);
        stringified_played_notes.push_str(" ");
    }

    // Add lower octave of notes
    stringified_played_notes.push_str("-|-");
    for note in played_notes.iter(){
        let mut note_base = note.clone();
        let mut note_octave_str = note_base.split_off(note.len() - 1);
        let note_octave = note_octave_str.parse::<i16>().unwrap_or(0) - 1;
        note_octave_str = note_octave.max(0).to_string();

        stringified_played_notes.push_str(&note_base);
        stringified_played_notes.push_str(&note_octave_str);
        stringified_played_notes.push_str(" ");
    }

    for (songname, melody) in songbook.iter() {
        if stringified_played_notes.contains(melody) {
            played_notes.clear();
            played_notes.push_back(songname.clone());
        }
    }
}