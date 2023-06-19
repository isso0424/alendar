use bitflags::bitflags;

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SimpleNote {
    Top,
    Middle,
    Base,
}

#[allow(unused)]
#[derive(PartialEq, Eq, Clone)]
pub enum Note {
    Simple(SimpleNote),
    TopAndMiddle,
    MiddleAndBase,
}

#[allow(unused)]
impl Note {
    fn satisfy(&self, note: SimpleNote) -> bool {
        match self {
            Self::Simple(n) => &note == n,
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
        let mut tmp = *self >> Self::One;
        if (self.intersects(Self::One)) {
            tmp = tmp.add(Self::Max);
        }

        *self = tmp;
    }

    fn l_shift(&mut self) {
        let mut tmp = *self << Self::One;
        if (self.intersects(Self::Max)) {
            tmp = tmp.add(Self::One);
        }

        *self = tmp;
    }

    fn distance(&self, family: Self) -> usize {
        let mut i = 0;
        let mut l = *self;
        let mut r = *self;

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
#[derive(Clone)]
pub struct EssentialOil {
    name: String,
    note: Note,
    family: Family,
    // TODO: add effect
    // TODO: add remain
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

    pub fn satisfy_note(&self, note: SimpleNote) -> bool {
        self.note.satisfy(note)
    }

    pub fn satisfy_family(&self, family: Family) -> bool {
        self.family.satisfy(family)
    }

    pub fn compatible_family(&self, family: Family, threshold: usize) -> bool {
        self.family.distance(family) <= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_distance() {
        let c = Family::Citrus;
        let h = Family::Herball;
        let e = Family::Earthy;

        assert_eq!(c.distance(h), 2);
        assert_eq!(c.distance(e), 1);

        let brend_e_h = e.add(h);
        assert_eq!(brend_e_h.distance(c), 1);
        assert_eq!(brend_e_h.distance(e), 0);
    }
}
