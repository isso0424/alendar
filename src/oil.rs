use bitflags::bitflags;

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SimpleNote {
    Top,
    Middle,
    Base,
}

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Note {
    Simple(SimpleNote),
    TopAndMiddle,
    MiddleAndBase,
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Strength {
    Week,
    Middle,
    Strong,
}

#[allow(unused)]
impl Strength {
    fn recommended_amount(&self) -> u8 {
        match self {
            Self::Week => 4,
            Self::Middle => 2,
            Self::Strong => 1,
        }
    }
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

    fn simplify(&self) -> Vec<SimpleNote> {
        let mut v = vec![];

        match self {
            Self::Simple(note) => v.push(*note),
            Self::TopAndMiddle => {
                v.push(SimpleNote::Top);
                v.push(SimpleNote::Middle);
            }
            Self::MiddleAndBase => {
                v.push(SimpleNote::Middle);
                v.push(SimpleNote::Base);
            }
        };

        v
    }
}

bitflags! {
    #[derive(Clone, Copy, Default, Debug)]
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
#[derive(Clone, Debug)]
pub struct EssentialOil {
    pub id: uuid::Uuid,
    pub name: String,
    pub note: Note,
    pub family: Family,
    pub strength: Strength,
    pub remaining_amount: u8,
    // TODO: add effect
    // TODO: add remain
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct BlendedElement {
    oil: EssentialOil,
    amount: u8,
}

#[allow(unused)]
impl EssentialOil {
    pub fn new(
        id: uuid::Uuid,
        name: &str,
        note: Note,
        family: Family,
        strength: Strength,
        remaining_amount: u8,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            note,
            family,
            strength,
            remaining_amount,
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

    pub fn blend(lhs: &Self, left_amount: u8, rhs: &Self, right_amount: u8) -> BlendedOil {
        BlendedOil {
            oils: vec![
                BlendedElement {
                    oil: lhs.clone(),
                    amount: left_amount,
                },
                BlendedElement {
                    oil: rhs.clone(),
                    amount: right_amount,
                },
            ],
        }
    }

    pub fn recommended_amount(&self) -> u8 {
        self.strength.recommended_amount()
    }
}

#[derive(Clone, Debug)]
pub struct BlendedOil {
    oils: Vec<BlendedElement>,
}

#[allow(unused)]
impl BlendedOil {
    pub fn missing_notes(&self) -> Vec<SimpleNote> {
        let mut exists_notes = self.oils.clone().into_iter().flat_map(|o| {
            o.oil
                .note
                .simplify()
                .into_iter()
                .collect::<std::collections::HashSet<SimpleNote>>()
        });

        vec![SimpleNote::Top, SimpleNote::Middle, SimpleNote::Base]
            .into_iter()
            .filter(|n| exists_notes.all(|e| e != *n))
            .collect()
    }

    pub fn compatible_family(&self, family: Family, threshold: usize) -> bool {
        self.oils
            .clone()
            .into_iter()
            .map(|o| o.oil.family.distance(family))
            .min()
            .unwrap()
            <= threshold
    }

    pub fn blend(&self, oil: &EssentialOil, amount: u8) -> BlendedOil {
        let mut oils = self.oils.clone();
        oils.push(BlendedElement {
            oil: oil.clone(),
            amount,
        });

        BlendedOil { oils }
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

    #[test]
    fn test_blend_oils() {
        let c = EssentialOil::new(
            uuid::Uuid::new_v4(),
            "test_c",
            Note::TopAndMiddle,
            Family::Citrus,
            Strength::Week,
            50,
        );
        let h = EssentialOil::new(
            uuid::Uuid::new_v4(),
            "test_h",
            Note::Simple(SimpleNote::Middle),
            Family::Herball,
            Strength::Middle,
            50,
        );

        assert_eq!(c.recommended_amount(), 4);
        assert_eq!(h.recommended_amount(), 2);

        let blended = EssentialOil::blend(&c, 2, &h, 3);

        assert_eq!(*blended.missing_notes().get(0).unwrap(), SimpleNote::Base);

        assert_eq!(blended.oils.get(0).unwrap().amount, 2);
        assert_eq!(blended.oils.get(0).unwrap().oil.name, "test_c");
        assert_eq!(blended.oils.get(1).unwrap().amount, 3);
        assert_eq!(blended.oils.get(1).unwrap().oil.name, "test_h");
    }
}
