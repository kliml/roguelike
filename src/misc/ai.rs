// use crate::Map;
// use crate::Object;
// use crate::Tcod;
// use crate::Game;
// use crate::PLAYER;

use crate::*;
use crate::misc::help::mut_two;

pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut Vec<Object>) {
  // Vector from object to targer
  let dx = target_x - objects[id].x;
  let dy = target_y - objects[id].y;
  let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

  let dx = (dx as f32 / distance).round() as i32;
  let dy = (dy as f32 / distance).round() as i32;
  map::move_by(id, dx, dy, map, objects);
}

pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut Vec<Object>) {
  // Monster takes turn if is in fov
  let (monster_x, monster_y) = objects[monster_id].pos();
  if tcod.fov.is_in_fov(monster_x, monster_y) {
    // Move or attack depending on distance
    if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
      let (player_x, player_y) = objects[PLAYER].pos();
      move_towards(monster_id, player_x, player_y, &game.map, objects);
    } else {
      let (monster, player) = mut_two(monster_id, PLAYER, objects);
      monster.attack(player, game);
    }
  }
}
