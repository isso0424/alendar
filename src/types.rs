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
    #[derive(Clone, Copy, Default)]
    pub struct Family: u8 {
        const One     = 0b0000001;
        const Citrus  = 0b0000001;
        const Froral =  0b0000010;
        const Herball = 0b0000100;
        const Wood    = 0b0001000;
        const Resin =   0b0010000;
        const Spicy =   0b0100000;
        const Earthy =  0b1000000;
        const Max     = 0b1000000;
    }
}

impl From<u8> for Family {
    fn from(item: u8) -> Self {
        Family::from_bits_retain(item)
    }
}

impl std::ops::Shr for Family {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self {
        Family::from(self.bits() >> rhs.bits())
    }
}

impl std::ops::Shl for Family {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self {
        Family::from(self.bits() << rhs.bits())
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

    fn r_shift(&mut self) {
        let mut tmp = self.clone() >> Self::One;
        if (self.intersects(Self::One)) {
            tmp = tmp.add(Self::Max);
        }

        *self = tmp;
    }

    fn l_shift(&mut self) {
        let mut tmp = self.clone() << Self::One;
        if (self.intersects(Self::Max)) {
            tmp = tmp.add(Self::One);
        }

        *self = tmp;
    }

    fn distance(&self, family: Self) -> usize {
        let mut i = 0;
        let mut l = self.clone();
        let mut r = self.clone();

        loop {
            if (l.intersects(family) || r.intersects(family)) {
                return i;
            }
            l.l_shift();
            r.r_shift();
            i += 1;
        }
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
