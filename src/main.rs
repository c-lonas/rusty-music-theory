use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!("Error: Must supply one (and only one) argument that is a valid Key");
        process::exit(1);
    }

    let note = &args[1];
    
    validate_user_input(note);

    let is_flats_key = determine_if_flats(note);

    println!("Using flats? {}", is_flats_key);
    
 
}

fn validate_user_input(note: &str) {
    let valid_notes = ["C", "G", "D", "A", "E", "B", "Cb", "Gb", "F#", "Db", "C#", "Ab", "Eb", "Bb", "F"];

    if !valid_notes.contains(&note) {
        println!("Error: '{}' is not a valid key", note);
        println!("Please supply one of the following valid arguments");
        println!("{:?}", valid_notes);

        process::exit(1);
    } 

}

fn determine_if_flats(note: &str) -> bool {
    
    let flats_keys = ["F", "Bb", "Eb", "Ab", "Db", "Gb", "Cb"];

    if flats_keys.contains(&note) {
        true
    } else {
        false
    }
}