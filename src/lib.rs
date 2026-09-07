use rand::*;
pub mod fights;
pub use fights::*;

#[derive(Debug)]
pub struct Chicken {
    health: f32,   // hp
    attack: f32,   // damage
    size: u32,     // size of animal
    speed: u32,    // speed of attack
}

#[derive(Debug)]
pub struct TRex {
    health: f32,   // hp
    attack: f32,   // damage
    size: u32,     // size of animal
    speed: u32,    // speed of attack
}

impl Chicken {
    pub fn new() -> Chicken {
        Chicken {
            health: random_range(8.0..12.0),
            attack: random_range(0.1..0.3),
            size: random_range(1..3), 
            speed: random_range(1..3)
        }
    }
    pub fn name() -> &'static str{
        "Chicken"
    }
}

impl TRex {
    pub fn new() -> Self {
        Self {
            health: random_range(1500.0..2000.0),
            attack: random_range(200.0..250.0),
            size: random_range(5000..10000), 
            speed: random_range(3..7),
        }
    }
    pub fn name() -> &'static str{
        "Trex"
    }
}

trait Animal:Sized {
    fn size(&self) -> u32;
    fn speed(&self) -> u32;
    fn attack(&self) -> f32;
    fn health(&self) -> f32;
    fn damage(&mut self, val:f32 );
    fn name() -> &'static str;
}

impl Animal for Chicken {
    fn size(&self) -> u32 {
        self.size
    }
    fn speed(&self) -> u32 {
        self.speed
    }
    fn attack(&self) -> f32 {
        self.attack
    }
    fn health(&self) -> f32 {
        self.health
    }
    fn damage(&mut self, val:f32 ) {
        self.health -= val;
    }
    fn name() -> &'static str {
        Self::name()
    }
}

impl Animal for TRex {
    fn size(&self) -> u32 {
        self.size
    }
    fn speed(&self) -> u32 {
        self.speed
    }
    fn attack(&self) -> f32 {
        self.attack
    }
    fn health(&self) -> f32 {
        self.health
    }
    fn damage(&mut self, val:f32 ) {
        self.health -= val;
    }
    fn name() -> &'static str {
        Self::name()
    }
}

