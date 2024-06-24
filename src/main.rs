#![allow(unused)]

use std::env;
use std::process;


#[derive(Debug, Clone)]
struct Note {
    name: String,
    index: usize,
}

#[derive(Debug, Clone)]
enum Scale {
    Major,
    Minor
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!("Error: Must supply one (and only one) argument that is a valid Key");
        process::exit(1);
    }

    let root_note = &args[1];
    
    validate_user_input(root_note);

    let is_flats_key = determine_if_flats(root_note);

    // Construct major scale 
    let scale_choice = Scale::Minor;
    let scale = construct_scale(root_note, scale_choice.clone(), is_flats_key); 
    
    println!("{:?} {:?} Scale:", root_note, scale_choice);
    for degree in scale {
        println!("{}", degree)
    }
 
}

fn validate_user_input(root_note: &str) {

    let valid_notes = ["C", "G", "D", "A", "E", "B", "Cb", "Gb", "F#", "Db", "C#", "Ab", "Eb", "Bb", "F"];
    if !valid_notes.contains(&root_note) {
        eprintln!("Error: '{}' is not a valid key", root_note);
        eprintln!("Please supply one of the following valid arguments");
        eprintln!("{:?}", valid_notes);

        process::exit(1);
    } 

}

fn determine_if_flats(root_note: &str) -> bool {
    
    let flats_keys = ["F", "Bb", "Eb", "Ab", "Db", "Gb", "Cb"];
    if flats_keys.contains(&root_note) {
        true
    } else {
        false
    }
}

fn construct_scale(root_note: &str, scale: Scale , is_flats_key: bool) -> Vec<String> {
    let notes_sharps = vec![
        ("C", 0), ("C#", 1), ("D", 2), ("D#", 3), ("E", 4), ("F", 5), ("F#", 6),
        ("G", 7), ("G#", 8), ("A", 9), ("A#", 10), ("B", 11),
    ];
    
    let notes_flats = vec![
        ("C", 0), ("Db", 1), ("D", 2), ("Eb", 3), ("E", 4), ("F", 5), ("Gb", 6),
        ("G", 7), ("Ab", 8), ("A", 9), ("Bb", 10), ("B", 11),
    ];

    let steps = match scale {
        Scale::Major => [2, 2, 1, 2, 2, 2, 1],
        Scale::Minor => [2, 1, 2, 2, 1, 2, 2],
    };
    
    let mut scale = Vec::new();

    let available_notes = if is_flats_key {
        &notes_flats
    } else {
        &notes_sharps
    };

    let mut current_index = available_notes.iter().position(|&(note, _)| note == root_note).unwrap();

    for step in steps.iter() {
        scale.push(available_notes[current_index].0.to_string());
        current_index = (current_index + step) % available_notes.len();
    }

    scale.push(available_notes[current_index].0.to_string());

    scale
}