#[derive(Clone)]
pub struct Wearable {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub rarity: String,
    pub body_shapes: Vec<String>,
}

impl Wearable {
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: String,
        rarity: String,
        body_shapes: Vec<String>,
    ) -> Self {
        Wearable {
            id,
            name,
            description,
            category,
            rarity,
            body_shapes,
        }
    }
}
