enum SimpleNote {
    Top,
    Middle,
    Base,
}

pub enum Note {
    SimpleNote(SimpleNote),
    TopAndMiddle,
    MiddleAndBase,
}

impl Note {
    fn satisfy(&self, note: SimpleNote) -> bool {
        match self {
            SimpleNote(n) => n == note,
            Self::TopAndMiddle => note == SimpleNote::Top || note == SimpleNote::Middle,
            Self::MiddleAndBase => note == SimpleNote::Middle || note == SimpleNote::Base,
        }
    }
}

pub struct EssentialOil {
    name: String,
    note: Note,
    // TODO: add type
    // TODO: add effect
}

impl EssentialOil {
    pub fn new(name: &str, note: Note) -> Self {
        Self {
            name: name.to_string(),
            note,
        }
    }
}
