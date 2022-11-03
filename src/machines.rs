use std::collections::HashMap;

pub struct Buffer {
    limits: HashMap<Resource, u32>,
    amount: HashMap<Resource, u32>,
}

pub struct Resource {
    name: String,
    description: String,
}
