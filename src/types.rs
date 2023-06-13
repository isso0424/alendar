use bitflags::bitflags;

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum SimpleNote {
    Top,
    Middle,
    Base,
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum Note {
    SimpleNote(SimpleNote),
    TopAndMiddle,
    MiddleAndBase,
}

#[allow(unused)]
impl Note {
    fn satisfy(&self, note: SimpleNote) -> bool {
        match self {
            Self::SimpleNote(n) => &note == n,
            Self::TopAndMiddle => note == SimpleNote::Top || note == SimpleNote::Middle,
            Self::MiddleAndBase => note == SimpleNote::Middle || note == SimpleNote::Base,
        }
    }
}

bitflags! {
    #[derive(Clone)]
    pub struct Family: u8 {
        const Woody =   0b00000001;
        const Froral =  0b00000010;
        const Resin =   0b00000100;
        const Herball = 0b00001000;
        const Spicy =   0b00010000;
        const Citrus  = 0b00100000;
        const Fruity  = 0b01000000;
    }
}

#[allow(unused)]
impl Family {
    fn add(&self, new: Self) -> Self {
        self.to_owned().union(new)
    }

    fn satisfy(&self, family: Self) -> bool {
        self.intersects(family)
    }
}

#[allow(unused)]
pub struct EssentialOil {
    name: String,
    note: Note,
    family: Family,
    // TODO: add effect
}

#[allow(unused)]
impl EssentialOil {
    pub fn new(name: &str, note: Note, family: Family) -> Self {
        Self {
            name: name.to_string(),
            note,
            family,
        }
    }
}
