pub struct Energy {
    pub name: String,
    pub value: i32,
    pub max: i32,
    pub inc_size: i32,
}

impl Energy {
    pub fn new() -> Energy {
        Energy {
            name: "energy".to_string(),
            value: 10,
            max: 100,
            inc_size: 1,
        }
    }    
}

pub struct Batteries {
    pub name: String,
    pub value: i32,
}

impl Batteries {
    pub fn new() -> Batteries {
        Batteries {
            name: "batteries".to_string(),
            value: 10,
        }
    }
}
