use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Note {
    name: String,
    index: usize,
}

impl Note {
    fn new(name: &str, index: usize) -> Note {
        Note {
            name: name.to_string(),
            index,
        }
    }

    fn semitone_up(&self, steps: usize, use_sharps: bool) -> Note {
        let notes_sharps = vec![
            ("C", 0), ("C#", 1), ("D", 2), ("D#", 3), ("E", 4), ("F", 5), ("F#", 6),
            ("G", 7), ("G#", 8), ("A", 9), ("A#", 10), ("B", 11),
        ];
        let notes_flats = vec![
            ("C", 0), ("Db", 1), ("D", 2), ("Eb", 3), ("E", 4), ("F", 5), ("Gb", 6),
            ("G", 7), ("Ab", 8), ("A", 9), ("Bb", 10), ("B", 11),
        ];

        let notes = if use_sharps {
            &notes_sharps
        } else {
            &notes_flats
        };

        let new_index = (self.index + steps) % 12;
        let new_note = notes.iter().find(|&&(_, idx)| idx == new_index).unwrap();
        Note::new(new_note.0, new_note.1)
    }
}

fn use_sharps(root: &Note) -> bool {
    let sharps_keys = vec![0, 2, 4, 6, 7, 9, 11]; // C, D, E, F#, G, A, B
    sharps_keys.contains(&root.index)
}

fn construct_major_scale(root: Note) -> Vec<Note> {
    let steps = [2, 2, 1, 2, 2, 2, 1];
    let mut scale = Vec::new();
    let mut current_note = root.clone();
    let use_sharps = use_sharps(&root);

    scale.push(current_note.clone());
    for &step in &steps {
        current_note = current_note.semitone_up(step, use_sharps);
        scale.push(current_note.clone());
    }

    scale
}

fn main() {
    let root = Note::new("Eb", 3); 
    let scale = construct_major_scale(root);
    for note in scale {
        println!("Note: {}", note.name);
    }
}
