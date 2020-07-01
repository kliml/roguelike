use tcod::colors::*;

use crate::object::{Effect, Object, StatusEffect};
use crate::settings::*;
use crate::{closest_monster, Game, Tcod, UseResult};

pub enum Spells {
    Heal,
    Lightning,
    Freeze,
}

const HEAL_AMOUNT: i32 = 4;
const HEAL_MANA_COST: i32 = 5;
fn cast_heal(_tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if let Some(mut fighter) = objects[PLAYER].fighter {
        if fighter.mana >= HEAL_MANA_COST {
            if fighter.hp == fighter.max_hp {
                game.messages.add("You are already at full health.", RED);
                return UseResult::Cancelled;
            }
            game.messages
                .add("Your wounds start to feel better!", LIGHT_VIOLET);

            // Reduce mana
            fighter.mana -= HEAL_MANA_COST;
            objects[PLAYER].fighter = Some(fighter);

            // Apply heal
            objects[PLAYER].heal(HEAL_AMOUNT);
            return UseResult::UsedUp;
        } else {
            game.messages.add("Not enough mana to cast Heal", RED);
            return UseResult::Cancelled;
        }
    }
    UseResult::Cancelled
}

const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;
const LIGHTNING_MANA_COST: i32 = 10;
fn cast_lightning(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if let Some(mut fighter) = objects[PLAYER].fighter {
        if fighter.mana >= LIGHTNING_MANA_COST {
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
                // Apply damage
                objects[monster_id].take_damage(LIGHTNING_DAMAGE, game);
                // Reduce mana
                fighter.mana -= LIGHTNING_MANA_COST;
                objects[PLAYER].fighter = Some(fighter);
                return UseResult::UsedUp;
            } else {
                game.messages.add("No enemy is close enough to strike", RED);
                return UseResult::Cancelled;
            }
        } else {
            game.messages.add("Not enough mana to cast Lightning", RED);
            return UseResult::Cancelled;
        }
    }
    UseResult::Cancelled
}

const FREEZE_RADIUS: i32 = 4;
const FREEZE_DURATION: i32 = 3;
const FREEZE_MANA_COST: i32 = 5;
fn cast_freeze(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> UseResult {
    if let Some(mut fighter) = objects[PLAYER].fighter {
        if fighter.mana >= FREEZE_MANA_COST {
            let monster_id = closest_monster(tcod, objects, LIGHTNING_RANGE);
            if let Some(monster_id) = monster_id {
                game.messages.add(
                    format!(
                        "{} becomes frozen for {} turns!",
                        objects[monster_id].name, FREEZE_DURATION
                    ),
                    LIGHT_BLUE,
                );
                // Apply freeze
                objects[monster_id].effect = Some(StatusEffect {
                    effect: Effect::Frozen,
                    turns_left: FREEZE_DURATION,
                });
                // Reduce mana
                fighter.mana -= FREEZE_MANA_COST;
                objects[PLAYER].fighter = Some(fighter);
                return UseResult::UsedUp;
            } else {
                game.messages.add("No enemy is close enough to Freeze", RED);
                return UseResult::Cancelled;
            }
        } else {
            game.messages.add("Not enough mana to cast Freeze", RED);
            return UseResult::Cancelled;
        }
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
    }
}
