use rand::*;
pub mod fights;
pub use fights::*;

#[derive(Debug)]
pub struct Chicken {
    id: usize,     // identifier
    health: f32,   // hp
    attack: f32,   // damage
    size: u32,     // size of animal
    speed: u32,    // speed of attack
}

#[derive(Debug)]
pub struct TRex {
    id: usize,     // identifier
    health: f32,   // hp
    attack: f32,   // damage
    size: u32,     // size of animal
    speed: u32,    // speed of attack
}

impl Chicken {
    pub fn new(id: usize) -> Chicken {
        Chicken {
            id,
            health: random_range(8.0..12.0),
            attack: random_range(0.5..2.5),
            size: random_range(1..3), 
            speed: random_range(50..90),
        }
    }
}

impl TRex {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            health: random_range(1500.0..2000.0),
            attack: random_range(300.0..500.0),
            size: random_range(1000..2000), 
            speed: random_range(5..10),
        }
    }
}

#[derive(Debug)]
enum Attacker {
    Chicken(usize), // chicken id
    TRex(usize),    // trex id
}

struct AttackEvent {
    time: u32,
    attacker: Attacker,
}
