pub mod menu;
pub mod messages;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use tcod::colors::*;
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};

use serde::{Deserialize, Serialize};

use crate::ai;
use crate::map;
use crate::object;
use crate::settings::*;
//use crate::Game;
use crate::Tcod;

use crate::handle_keys;
use crate::render_all;
use crate::PlayerAction;

use map::Map;
use object::DeathCallback;
use object::Fighter;
use object::Object;

use messages::*;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub map: Map,
    pub messages: Messages,
    pub inventory: Vec<Object>,
}

pub fn new_game(tcod: &mut Tcod) -> (Game, Vec<Object>) {
    // Create player
    let mut player = Object::new(0, 0, '@', WHITE, "player", true);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp: 30,
        hp: 30,
        max_mana: 30,
        mana: 30,
        defense: 2,
        power: 5,
        on_death: DeathCallback::Player,
    });

    let mut objects = vec![player];

    let mut game = Game {
        map: map::make_map(&mut objects),
        messages: Messages::new(),
        inventory: vec![],
    };

    initialise_fov(tcod, &game.map);

    // Warm welcome message
    game.messages.add("Welcome traveller! Prepare to die!", RED);

    (game, objects)
}

pub fn initialise_fov(tcod: &mut Tcod, map: &Map) {
    for x in 0..map::MAP_WIDTH {
        for y in 0..map::MAP_HEIGHT {
            tcod.fov.set(
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }
}

pub fn play_game(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        tcod.con.clear();

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }

        let fov_recompute = previous_player_position != objects[PLAYER].pos();
        render_all(tcod, game, objects, fov_recompute);

        tcod.root.flush();

        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(tcod, game, objects);
        if player_action == PlayerAction::Exit {
            save_game(game, objects).unwrap();
            break;
        }

        if objects[PLAYER].alive && player_action == PlayerAction::TookTurn {
            for id in 1..objects.len() {
                if objects[id].ai.is_some() {
                    ai::ai_take_turn(id, tcod, game, objects);
                }
            }
        }
    }
}

fn save_game(game: &Game, objects: &Vec<Object>) -> Result<(), Box<dyn Error>> {
    let save_data = serde_json::to_string(&(game, objects))?;
    let mut file = File::create("savegame")?;
    file.write_all(save_data.as_bytes())?;
    Ok(())
}

pub fn load_game() -> Result<(Game, Vec<Object>), Box<dyn Error>> {
    let mut json_save_state = String::new();
    let mut file = File::open("savegame")?;
    file.read_to_string(&mut json_save_state)?;
    let result = serde_json::from_str::<(Game, Vec<Object>)>(&json_save_state)?;
    Ok(result)
}
