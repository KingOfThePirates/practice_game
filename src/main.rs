use std::mem;



const GAME_NAME: &'static str = "Fighter";



// ======================================================================
// New types
//
type Health = f64;
type Attack = f64;
type Level = u8;
type Experience = f64;
// ======================================================================



// ======================================================================
// Abstractions
//
#[derive(Clone)]
struct Resources {
    player: Box<Hero>,
    enemy: Box<Unit>,
}

impl Resources {
    pub fn realize_player(&self) -> Box<Hero> {
        let cloned_self = self.clone();
        cloned_self.player
    }

    pub fn realize_enemy(&self) -> Box<Unit> {
        let cloned_self = self.clone();
        cloned_self.enemy
    }
}

#[derive(Clone)]
struct Unit {
    hp: Health,
    atk: Attack,
}

impl Unit {
    fn new() -> Box<Unit> {
        Box::new(Unit {
            hp: 100.0,
            atk: 10.0,
        })
    }

    pub fn show(&self) -> String {
        format!("{}hp {}atk",
                self.realize_hp() as i32,
                self.realize_atk() as i32)
    }

    pub fn change_current_hp(&mut self, amount: f64) {
        self.hp += amount;
    }

    pub fn realize_hp(&self) -> f64 {
        self.hp
    }

    pub fn realize_atk(&self) -> f64 {
        self.atk
    }
}

#[derive(Clone)]
struct Hero {
    hp: Health,
    atk: Attack,
    lvl: Level,
    exp: Experience,
}

impl Hero {
    fn new() -> Box<Hero> {
        Box::new(Hero {
            hp: 200.0,
            atk: 20.0,
            lvl: 1,
            exp: 0.0,
        })
    }

    pub fn show(&self) -> String {
        format!("{}hp {}atk",
                self.hp as i32,
                self.atk as i32)
    }

    pub fn change_current_hp(&mut self, amount: f64) {
        self.hp += amount;
    }

    pub fn realize_hp(&self) -> f64 {
        self.hp
    }

    pub fn realize_atk(&self) -> f64 {
        self.atk
    }

    pub fn realize_lvl(&self) -> u8 {
        self.lvl
    }

    pub fn realize_exp(&self) -> f64 {
        self.exp
    }
}
// ======================================================================










// ======================================================================
//
fn main() {
    let mut game_data: Box<Resources> = Box::new(Resources {
        player: Hero::new(),
        enemy: Unit::new(),
    });
    println!("Welcome to {}!", GAME_NAME);
    draw_line_break();
    draw_line_break();
    println!("An enemy appears!");
    battle(&mut game_data);
    battle(&mut game_data);
    show_mem(&game_data);


}
// ======================================================================











// ======================================================================
// Helper Routines
//
#[allow(dead_code)]
fn show_mem(in_game_data: &Resources) {
    let ref enemy = in_game_data.realize_enemy();
    let ref player = in_game_data.realize_player();

    println!("resource    bytes in the stack");
    println!("GAME_NAME:  {}", mem::size_of_val(&GAME_NAME));
    println!("game_data:  {}", mem::size_of_val(&in_game_data));
    println!("enemy:      {}", mem::size_of_val(&enemy));
    println!("enemy.hp:   {}", mem::size_of_val(&enemy.realize_hp()));
    println!("enemy.atk:  {}", mem::size_of_val(&enemy.realize_atk()));
    println!("player:     {}", mem::size_of_val(&player));
    println!("player.hp:  {}", mem::size_of_val(&player.realize_hp()));
    println!("player.atk: {}", mem::size_of_val(&player.realize_atk()));
    println!("player.lvl: {}", mem::size_of_val(&player.realize_lvl()));
    println!("player.exp: {}", mem::size_of_val(&player.realize_exp()));
}

fn draw_line_break() {
    println!("");
}

fn battle(in_game_data: &mut Box<Resources>) {
    let mut game_data: &mut Box<Resources> = in_game_data;
    draw_hp_bars(&game_data);
    read_user_input();
    deal_damage(&mut game_data);
    draw_combat_text(&game_data);
    draw_line_break();
    draw_line_break();
}

fn read_user_input() {
    let user_input: &str = &"attack";
    println!("{}", user_input);
}

fn deal_damage(in_game_data: &mut Box<Resources>) {
    let cloned_game_data = in_game_data.clone();
    let ref player = cloned_game_data.player;
    let ref mut enemy = in_game_data.enemy;

    let dmg = -player.realize_atk();
    enemy.change_current_hp(dmg);
}

fn draw_hp_bars(in_game_data: &Resources) {
    let ref enemy = in_game_data.enemy;
    let ref player = in_game_data.player;

    println!("--------------------");
    println!("enemy: {}", enemy.show());
    println!("player: {}", player.show());
    println!("--------------------");
}

fn draw_combat_text(in_game_data: &Resources) {
    let ref enemy = in_game_data.enemy;
    let ref player = in_game_data.player;

    let old_health: f64 = enemy.realize_hp() + player.realize_atk();
    let new_health: f64 = enemy.realize_hp();
    let damage: f64 = player.realize_atk();
    print!("{}hp -> ", old_health as i32);
    print!("{}hp ", new_health as i32);
    println!("{}!", damage as i32);
}
// ======================================================================
