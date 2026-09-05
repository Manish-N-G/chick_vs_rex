use rand::*;

fn main() {
    // entry point
    let chick1 = Chicken::new();
    let chick2 = Chicken::new();

    let rex1 = TRex::new();
}

struct Chicken {
    health: f32,
    attack: f32,
}

struct TRex {
    health: f32,
    attack: f32,
}

impl Chicken {
    fn new() -> Chicken {
        Chicken {
            health: random_range(8.0..12.0),
            attack: random_range(0.5..2.5),
        }
    }
}

impl TRex {
    fn new() -> Self {
        Self {
            health: random_range(1500.0..2000.0),
            attack: random_range(300.0..500.0),
        }
    }
}
