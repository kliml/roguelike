use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};
use tcod::colors::*;

use crate::object::{Effect, Object, StatusEffect, Fighter, DeathCallback, UseResult};
use crate::settings::*;
use crate::{closest_monster, Game, Tcod};
use crate::help::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Spells {
    Heal,
    Lightning,
    Freeze,
    Fireball,
    Wall,
}

impl fmt::Display for Spells {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const HEAL_AMOUNT: i32 = 4;
const HEAL_MANA_COST: i32 = 5;
fn cast_heal(_tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if objects[PLAYER].fighter.map_or(0, |f| f.mana) >= HEAL_MANA_COST {
        let fighter = objects[PLAYER].fighter.as_mut().unwrap();
        if fighter.hp == fighter.max_hp {
            game.messages.add("You are already at full health.", RED);
            return UseResult::Cancelled;
        }
        game.messages
            .add("Your wounds start to feel better!", LIGHT_VIOLET);

        // Reduce mana
        fighter.mana -= HEAL_MANA_COST;
        // Apply heal
        objects[PLAYER].heal(HEAL_AMOUNT);
        return UseResult::UsedUp;
    }
    game.messages.add("Not enough mana to cast Heal", RED);
    UseResult::Cancelled
}

const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;
const LIGHTNING_MANA_COST: i32 = 10;
fn cast_lightning(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if objects[PLAYER].fighter.map_or(0, |f| f.mana) >= LIGHTNING_MANA_COST {
        let monster_id = closest_monster(tcod, objects, LIGHTNING_RANGE);
        if let Some(monster_id) = monster_id {
            game.messages.add(
                format!(
                    "A lightning bolt strikes the {} with a loud thunder! \
                    The damage is {} hit points.",
                    objects[monster_id].name, LIGHTNING_DAMAGE
                ),
                LIGHT_BLUE,
            );
            // Apply damage and get xp
            if let Some(xp) = objects[monster_id].take_damage(LIGHTNING_DAMAGE, game) {
                objects[PLAYER].fighter.as_mut().unwrap().xp += xp;
            };
            // Reduce mana
            objects[PLAYER].fighter.as_mut().unwrap().mana -= LIGHTNING_MANA_COST;
            return UseResult::UsedUp;
        } else {
            game.messages.add("No enemy is close enough to strike", RED);
            return UseResult::Cancelled;
        }
    }
    game.messages.add("Not enough mana to cast Lightning", RED);
    UseResult::Cancelled
}

const FREEZE_RADIUS: i32 = 4;
const FREEZE_DURATION: i32 = 3;
const FREEZE_MANA_COST: i32 = 5;
fn cast_freeze(_tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if objects[PLAYER].fighter.map_or(0, |f| f.mana) >= FREEZE_MANA_COST {
        game.messages
            .add("Temperature around you begins to drop...", LIGHT_BLUE);

        let (x, y) = objects[PLAYER].pos();

        for i in 1..objects.len() {
            if objects[i].distance(x, y) <= FREEZE_RADIUS as f32 && objects[i].fighter.is_some() {
                game.messages.add(
                    format!(
                        "{} becomes frozen for {} turns!",
                        objects[i].name, FREEZE_DURATION
                    ),
                    LIGHT_BLUE,
                );
                // Apply freeze
                objects[i].effect = Some(StatusEffect {
                    effect: Effect::Frozen,
                    turns_left: FREEZE_DURATION,
                });
            }
        }
        // Reduce mana
        objects[PLAYER].fighter.as_mut().unwrap().mana -= FREEZE_MANA_COST;
        return UseResult::UsedUp;
    }
    game.messages.add("Not enough mana to cast Freeze", RED);
    UseResult::Cancelled
}

const FIREBALL_RADIUS: i32 = 3;
const FIREBALL_DAMAGE: i32 = 12;
const FIREBALL_MANA_COST: i32 = 7;
fn cast_fireball(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if objects[PLAYER].fighter.map_or(0, |f| f.mana) >= FIREBALL_MANA_COST {
        game.messages.add(
            "Left-click a target tile for the Fireball, or right-click to cancel.",
            LIGHT_CYAN,
        );
        let (x, y) = match target_tile(tcod, game, objects, None) {
            Some(tile_pos) => tile_pos,
            None => return UseResult::Cancelled,
        };
        game.messages.add(
            format!(
                "The fireball explodes, burning everything within {} tiles!",
                FIREBALL_RADIUS
            ),
            ORANGE,
        );
        let mut xp_to_gain: i32 = 0;
        for obj in objects.iter_mut() {
            if obj.distance(x, y) <= FIREBALL_RADIUS as f32 && obj.fighter.is_some() {
                game.messages.add(
                    format!(
                        "The {} gets burned for {} hit points.",
                        obj.name, FIREBALL_DAMAGE
                    ),
                    ORANGE,
                );
                if let Some(xp) = obj.take_damage(FIREBALL_DAMAGE, game) {
                    xp_to_gain += xp;
                };
            }
        }
        objects[PLAYER].fighter.as_mut().unwrap().xp += xp_to_gain;
        return UseResult::UsedUp;
    }
    game.messages.add("Not enough mana to cast Fireball", RED);
    UseResult::Cancelled
}

const WALL_SIZE: i32 = 1;
const WALL_HP: i32 = 1;
const WALL_MANA_COST: i32 = 5;
fn cast_wall(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {

    if objects[PLAYER].fighter.map_or(0, |f| f.mana) >= WALL_MANA_COST {
        game.messages.add(
            "Left-click a target tile for the Wall, or right-click to cancel.",
            LIGHT_CYAN,
        );
        let (x, y) = match target_tile(tcod, game, objects, None) {
            Some(tile_pos) => tile_pos,
            None => return UseResult::Cancelled,
        };
        let mut wall = Object::new(x, y, 'w', BRASS, "wall", true);
        wall.fighter = Some(Fighter {
            max_hp: WALL_HP,
            hp: WALL_HP,
            max_mana: 0,
            mana: 0,
            defense: 0,
            power: 0,
            xp: 0,
            on_death: DeathCallback::Monster,
        });
        objects.push(wall);
        game.messages.add(
            format!(
                "The ground shaters, blocking several tiles!",
            ),
            BRASS,
        );
        objects[PLAYER].fighter.as_mut().unwrap().mana -= WALL_MANA_COST;
    }
    UseResult::Cancelled
}

pub fn cast_spell(
    spell: Spells,
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &mut Vec<Object>,
) -> UseResult {
    match spell {
        Spells::Heal => cast_heal(tcod, game, objects),
        Spells::Lightning => cast_lightning(tcod, game, objects),
        Spells::Freeze => cast_freeze(tcod, game, objects),
        Spells::Fireball => cast_fireball(tcod, game, objects),
        Spells::Wall => cast_wall(tcod, game, objects),
    }
}
