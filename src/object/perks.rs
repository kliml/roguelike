use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::object::Object;
use crate::settings::PLAYER;
use tcod::colors::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Perks {
    Scavenger,
}

const HEAL_AMMOUNT: i32 = 5;
pub fn trigger_scavenger(monster_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
    game.messages
        .add(format!("You ate {}.", objects[monster_id].name), YELLOW);
    objects.remove(monster_id);
    objects[PLAYER].heal(HEAL_AMMOUNT);
}
