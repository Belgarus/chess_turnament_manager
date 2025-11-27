#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub rating: Option<u32>,
}

impl Player {
    pub fn new(name: String, rating: Option<u32>) -> Self {
        Self { name, rating }
    }
}
