type Health = f64;
type Attack = f64;
type Level = u8;
type Experience = f64;

pub trait Unit {
    fn new(health: f64, attack: f64) -> Self;
    fn realize_hp(&self) -> f64;
    fn realize_atk(&self) -> f64;
    fn change_current_hp(&mut self, amount: f64);
    fn show(&self) -> String {
        format!("{}hp {}atk",
                self.realize_hp() as i32,
                self.realize_atk() as i32)
    }
}

#[derive(Clone)]
pub struct Hero {
    hp: Health,
    atk: Attack,
    lvl: Level,
    exp: Experience,
}

impl Unit for Hero {
    fn new(health: f64, attack: f64) -> Hero {
        Hero {
            hp: health,
            atk: attack,
            lvl: 1,
            exp: 0.0,
        }
    }
    fn realize_hp(&self) -> f64 { self.hp }
    fn realize_atk(&self) -> f64 { self.atk }
    fn change_current_hp(&mut self, amount: f64) { self.hp += amount; }
}

impl Hero {
    pub fn realize_lvl(&self) -> u8 { self.lvl }
    pub fn realize_exp(&self) -> f64 { self.exp }
}

#[derive(Clone)]
pub struct NonHero {
    hp: Health,
    atk: Attack,
}

impl Unit for NonHero {
    fn new(health: f64, attack: f64) -> NonHero {
        NonHero {
            hp: health,
            atk: attack,
        }
    }
    fn realize_hp(&self) -> f64 { self.hp }
    fn realize_atk(&self) -> f64 { self.atk }
    fn change_current_hp(&mut self, amount: f64) { self.hp += amount; }
}
