use tcod::input::Key;

use crate::game::menu;
use crate::game::{next_level, Game};
use crate::map;
use crate::object::items::{ use_item, drop_item };
use crate::object::{self, PlayerAction::*, *};
use crate::Tcod;
use crate::object::{ UseResult , perks::*};

use crate::settings::*;

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> PlayerAction {
    use tcod::input::KeyCode::*;

    let player_alive = objects[PLAYER].alive;
    match (tcod.key, tcod.key.text(), player_alive) {
        (
            Key {
                code: Enter,
                alt: true,
                ..
            },
            _,
            _,
        ) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }

        (Key { code: Escape, .. }, _, _) => Exit,
        (Key { code: Up, .. }, _, true) => {
            map::player_move_or_attack(0, -1, game, objects);
            TookTurn
        }
        (Key { code: Down, .. }, _, true) => {
            map::player_move_or_attack(0, 1, game, objects);
            TookTurn
        }
        (Key { code: Left, .. }, _, true) => {
            map::player_move_or_attack(-1, 0, game, objects);
            TookTurn
        }
        (Key { code: Right, .. }, _, true) => {
            map::player_move_or_attack(1, 0, game, objects);
            TookTurn
        }
        // Pick up item
        (Key { code: Text, .. }, "e", true) => {
            let item_id = objects
                .iter()
                .position(|obj| obj.pos() == objects[PLAYER].pos() && obj.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            DidntTakeTurn
        }
        // Inventory
        (Key { code: Text, .. }, "w", true) => {
            let inventory_index = menu::inventory_menu(
                &game.inventory,
                "Press the key next to an item to use it, or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                use_item(inventory_index, tcod, game, objects);
            }
            // maybe TookTurn
            DidntTakeTurn
        }
        // Drop item
        (Key { code: Text, ..}, "r", true) => {
            let inventory_index = menu::inventory_menu(
                &game.inventory,
                "Press the key next to an item to drop it, or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                drop_item(inventory_index, game, objects);
            }
            DidntTakeTurn
        }
        // Spells
        (Key { code: Text, .. }, "q", true) => {
            let spell_id = menu::spell_menu(
                &game.spells,
                "Press the key next to an spell to use it, or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(spell_id) = spell_id {
                let spell = game.spells[spell_id];
                match spells::cast_spell(spell, tcod, game, objects) {
                    UseResult::Cancelled => return DidntTakeTurn,
                    UseResult::UsedUp => return TookTurn,
                }
            }
            DidntTakeTurn
        }
        // Move to the next floor or consume corps
        (Key { code: Text, .. }, "f", true) => {
            let player_on_stairs = objects
                .iter()
                .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
            if player_on_stairs {
                next_level(tcod, game, objects);
                return DidntTakeTurn;
            }
            if game.perks.iter().any(|perk| perk == &Perks::Scavenger) {
                if let Some(dead_monster_id) = objects
                    .iter()
                    .position(|object| object.pos() == objects[PLAYER].pos() && object.char == '%') {
                        trigger_scavenger(dead_monster_id, game, objects);
                    }
            }
            DidntTakeTurn
        }
        // Character information
        (Key { code: Text, .. }, "t", _) => {
            let player = &objects[PLAYER];
            let level = player.level;
            let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
            let known_spells = game.spells.len();
            if let Some(fighter) = player.fighter.as_ref() {
                let msg = format!(
                    "Character information\n \
                    Level: {}\n \
                    Experience: {}\n \
                    Experience to level up: {}\n \
                    Maximum HP: {}\n \
                    Attack: {}\n \
                    Defense: {}\n \
                    Spells known: {}",
                    level,
                    fighter.xp,
                    level_up_xp,
                    fighter.max_hp,
                    fighter.power,
                    fighter.defense,
                    known_spells,
                );
                menu::msgbox(&msg, CHARACTER_SCREEN_WIDTH, &mut tcod.root);
            }
            DidntTakeTurn
        }
        // Cheats hehe
        (Key { code: Text, .. }, "m", _) => {
            for x in 0..map::MAP_WIDTH {
                for y in 0..map::MAP_HEIGHT {
                    game.map[x as usize][y as usize].explored = true;
                }
            }
            DidntTakeTurn
        }
        (Key { code: Text, ..}, "n", true) => {
            objects[PLAYER].fighter.as_mut().unwrap().xp += 100;
            DidntTakeTurn
        }

        _ => DidntTakeTurn,
    }
}
