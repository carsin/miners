use specs::error::NoError;
use bracket_terminal::prelude::RGB;
use specs::{prelude::*, Component, VecStorage, ConvertSaveload};

#[derive(PartialEq, Copy, Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: char,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {}

// generates lists of tiles visible from position and their light levels.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>, // positions relative to algorithm that are visible
    pub light_levels: Vec<Option<f32>>, // light levels
    pub emitter: Option<f32>, // determines if entity emits light, and if so the maximum strength of 1.0 to 0.0
    pub range: f32, // changes how deep the shadowcasting algorithm goes. affects fov viewrange & lightshed
    // pub max_strength: f32, // changes the light level at the source and thus how gradual the light shift is
    pub dirty: bool, // has game changed (player moved)?
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Monster {}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub armor: i32,
    pub damage: i32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct MeleeAttacking {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount : Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(recipient) = store.get_mut(victim) {
            recipient.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}
