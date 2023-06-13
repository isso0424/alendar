pub struct EssentialOil {
    name: String,
    // TODO: add note
    // TODO: add type
    // TODO: add effect
}

impl EssentialOil {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
