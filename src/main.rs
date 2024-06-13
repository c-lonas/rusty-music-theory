
struct Note {
    index: usize
}

impl Note {
    fn new(index: usize) -> Note {
        Note { index }
    }

    fn semitone_up(&mut self) {
        self.index = (self.index + 1) % 12;
    }
}

fn construct_major_scale(root: Note) -> Vec<Note> {
    // To Do
}


fn main() {
    println!("Hello, world!");
}

