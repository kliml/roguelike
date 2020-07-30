use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use crate::game::Game;
use crate::object::Object;
use crate::settings::PLAYER;
use tcod::colors::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Perks {
    Scavenger,
    MagicCannon,
    FireImmunity,
    FreezeImmunity,
}

impl fmt::Display for Perks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const HEAL_AMMOUNT: i32 = 5;
pub fn trigger_scavenger(monster_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    game.messages
        .add(format!("You ate {}.", objects[monster_id].name), YELLOW);
    objects.remove(monster_id);
    objects[PLAYER].heal(HEAL_AMMOUNT);
}

pub fn trigger_magic_cannon(game: &mut Game, objects: &mut Vec<Object>) {
    game.messages
        .add(format!("You start to feel things differently..."), VIOLET);
    let fighter = objects[PLAYER].fighter.as_mut().unwrap();
    fighter.max_mana *= 2;
    fighter.mana *= 2;
    fighter.hp = 10;
    fighter.max_hp = 10;
}
