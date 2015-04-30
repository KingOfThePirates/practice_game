extern crate practice_game;

// Game Objects
use practice_game::unit::{Unit, Hero, NonHero};

// Tests
use std::mem;



const GAME_NAME: &'static str = "Fighter";
const HERO_INIT_HEALTH: f64 = 200.0;
const HERO_INIT_ATTACK: f64 = 20.0;
const NONHERO_INIT_HEALTH: f64 = 100.0;
const NONHERO_INIT_ATTACK: f64 = 10.0;



// ======================================================================
// Abstractions
//
#[derive(Clone)]
pub struct Resources {
    player: Hero,
    enemy: NonHero,
}

impl Resources {
    pub fn realize_player(&self) -> Hero {
        let cloned_self = self.clone();
        cloned_self.player
    }

    pub fn realize_enemy(&self) -> NonHero {
        let cloned_self = self.clone();
        cloned_self.enemy
    }
}
// ======================================================================










// ======================================================================
//
fn main() {
    let mut game_data: Resources = Resources {
        player: Hero::new(HERO_INIT_HEALTH, HERO_INIT_ATTACK),
        enemy: NonHero::new(NONHERO_INIT_HEALTH, NONHERO_INIT_ATTACK),
    };
    println!("Welcome to {}!", GAME_NAME);
    draw_line_break();
    draw_line_break();
    println!("An enemy appears!");
    battle(&mut game_data);
    battle(&mut game_data);


}
// ======================================================================











// ======================================================================
// Helper Routines
//
// UI
//
pub fn draw_line_break() {
    println!("");
}

pub fn draw_hp_bars(in_game_data: &Resources) {
    let ref enemy = in_game_data.enemy;
    let ref player = in_game_data.player;

    println!("--------------------");
    println!("enemy: {}", enemy.show());
    println!("player: {}", player.show());
    println!("--------------------");
}

pub fn draw_combat_text(in_game_data: &Resources) {
    let ref enemy = in_game_data.enemy;
    let ref player = in_game_data.player;

    let old_health: f64 = enemy.realize_hp() + player.realize_atk();
    let new_health: f64 = enemy.realize_hp();
    let damage: f64 = player.realize_atk();
    print!("{}hp -> ", old_health as i32);
    print!("{}hp ", new_health as i32);
    println!("{}!", damage as i32);
}

// Memory
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

// Misc
//
fn battle(in_game_data: &mut Resources) {
    let mut game_data: &mut Resources = in_game_data;
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

fn deal_damage(in_game_data: &mut Resources) {
    let cloned_game_data = in_game_data.clone();
    let ref player = cloned_game_data.player;
    let ref mut enemy = in_game_data.enemy;

    let dmg = -player.realize_atk();
    enemy.change_current_hp(dmg);
}
// ======================================================================
