#[derive(PartialEq)]
#[derive(Debug)]
pub enum Buttons {
    BuyBattery,
    HireBatteryGuy,
}

pub struct Cost {
    pub kind: Buttons,
    // pub name: String,
    pub value: i32,
}
impl Cost {
    pub fn new(kind: Buttons, /* name: &str, */ value: i32) -> Cost {
        Cost {
            kind,
            // name: name.to_string(),
            value,
        }
    }
}

pub struct Costs {
    pub list: [Cost; 2],
}

impl Costs {
    pub fn new() -> Costs {
        Costs {
            list: [
                Cost::new(Buttons::BuyBattery, 80),
                Cost::new(Buttons::HireBatteryGuy, 120),
            ],
        }
    }
    pub fn get(&self, kind: &Buttons) -> i32 {
        self.list.iter().find(|x| x.kind == *kind).unwrap().value
    }

    pub fn can_afford(&self, have: i32, kind: &Buttons) -> bool {
        let res = have >= self.get(kind);
        res
    }
}
