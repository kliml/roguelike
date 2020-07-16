use serde::{Deserialize, Serialize};

use tcod::colors::*;
use crate::settings::PLAYER;
use crate::object::Object;
use crate::game::Game;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Perks {
  Scavenger,
}

pub fn trigger_scavenger(monster_id: usize, game: &mut Game, objects: &mut Vec<Object>) {
  game.messages
      .add(format!("You ate {}.", objects[monster_id].name), YELLOW);
}