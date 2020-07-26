use tcod::colors::*;

use crate::help::mut_two;
use crate::map;
use crate::settings::PLAYER;
use crate::Game;
use crate::Map;
use crate::Object;
use crate::Tcod;

use crate::object::Effect;

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
        // Check for status effect
        if let Some(mut effect) = objects[monster_id].effect {
            match (effect.effect, effect.turns_left) {
                (Effect::Frozen, 0) => {
                    objects[monster_id].color = DARK_ORANGE;
                    objects[monster_id].effect = None;
                    return;
                }
                (Effect::Frozen, _) => {
                    objects[monster_id].color = LIGHT_BLUE;
                    objects[monster_id].effect.as_mut().unwrap().turns_left -= 1;
                    return;
                }
                (Effect::Burning, 0) => {
                    objects[monster_id].color = DARK_ORANGE;
                    objects[monster_id].effect = None;
                    return;
                }
                (Effect::Burning, _) => {
                    objects[monster_id].color = AMBER;
                    objects[monster_id].take_damage(1, game);
                    objects[monster_id].effect.as_mut().unwrap().turns_left -= 1;
                    default(monster_id, tcod, game, objects);
                    return;
                }
            }
        }

        // // Move or attack depending on distance
        // if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
        //     let (player_x, player_y) = objects[PLAYER].pos();
        //     move_towards(monster_id, player_x, player_y, &game.map, objects);
        // } else {
        //     let (monster, player) = mut_two(monster_id, PLAYER, objects);
        //     monster.attack(player, game);
        // }
        default(monster_id, tcod, game, objects);
    }
}

pub fn default(monster_id: usize, _tcod: &Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    // Move or attack depending on distance
    if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
        let (player_x, player_y) = objects[PLAYER].pos();
        move_towards(monster_id, player_x, player_y, &game.map, objects);
    } else {
        let (monster, player) = mut_two(monster_id, PLAYER, objects);
        monster.attack(player, game);
    }
}
