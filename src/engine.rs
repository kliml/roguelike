use tcod::input::{self, Event, Key, Mouse};

use crate::game::menu;
use crate::game::{next_level, Game};
use crate::map;
use crate::object::{self, *};
use crate::PlayerAction::{self, *};
use crate::UseResult::{self, *};
use crate::{use_item, Tcod};

use crate::settings::PLAYER;

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
        (Key { code: Text, .. }, "g", true) => {
            let item_id = objects
                .iter()
                .position(|obj| obj.pos() == objects[PLAYER].pos() && obj.item.is_some());
            if let Some(item_id) = item_id {
                pick_item_up(item_id, game, objects);
            }
            DidntTakeTurn
        }
        (Key { code: Text, .. }, "i", true) => {
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
        (Key { code: Text, .. }, "s", true) => {
            let spell_id = menu::spell_menu(&mut tcod.root);
            if let Some(spell_id) = spell_id {
                use object::spells::*;
                let spell = match spell_id {
                    0 => Spells::Heal,
                    1 => Spells::Lightning,
                    2 => Spells::Freeze,
                    _ => return DidntTakeTurn,
                };
                match cast_spell(spell, tcod, game, objects) {
                    UseResult::Cancelled => return DidntTakeTurn,
                    UseResult::UsedUp => return TookTurn,
                }
            }
            DidntTakeTurn
        }
        (Key { code: Text, .. }, "d", true) => {
            let player_on_stairs = objects
                .iter()
                .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
            if player_on_stairs {
                next_level(tcod, game, objects);
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

        _ => DidntTakeTurn,
    }
}
