use crate::oil::{EssentialOil, Family, SimpleNote};

#[allow(unused)]
struct Blender {
    oils: Vec<EssentialOil>,
}

#[allow(unused)]
impl Blender {
    fn new(oils: Vec<EssentialOil>) -> Self {
        Blender { oils }
    }

    fn search_by_note(&self, note: SimpleNote) -> Vec<EssentialOil> {
        self.oils
            .clone()
            .into_iter()
            .filter(|o| o.satisfy_note(note))
            .collect()
    }

    fn search_by_family(&self, family: Family) -> Vec<EssentialOil> {
        self.oils
            .clone()
            .into_iter()
            .filter(|o| o.satisfy_family(family))
            .collect()
    }

    fn search_by_similar(&self, family: Family, threshold: usize) -> Vec<EssentialOil> {
        self.oils
            .clone()
            .into_iter()
            .filter(|o| o.compatible_family(family, threshold))
            .collect()
    }
}
