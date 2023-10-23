use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Basic {
    name: String,
}
impl Basic {
    pub fn new() -> Self {
        Self {
            name: "John Doe".to_string(),
        }
    }
}

impl Default for Basic {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Details {
    age: u8,
    height: u8,
    likes: [String; 3],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Intermediate {
    name: String,
    details: Details,
}

impl Default for Intermediate {
    fn default() -> Self {
        Self::new()
    }
}

impl Intermediate {
    pub fn new() -> Self {
        Self {
            name: "John Doe".to_string(),
            details: Details {
                age: 25,
                height: 186,
                likes: [
                    "cheese".to_string(),
                    "the color blue".to_string(),
                    "rock music".to_string(),
                ],
            },
        }
    }
}
