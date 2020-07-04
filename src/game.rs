pub mod menu;
pub mod messages;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use tcod::colors::*;
use tcod::console::*;
use tcod::input::{self, Event};

use serde::{Deserialize, Serialize};

use crate::ai;
use crate::engine::handle_keys;
use crate::map;
use crate::object;
use crate::renderer::*;
use crate::settings::*;
use crate::PlayerAction;
use crate::Tcod;
use object::spells::Spells;

use map::make_map;
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
    pub dungeon_level: u32,
    pub spells: Vec<Spells>,
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
        xp: 0,
        on_death: DeathCallback::Player,
    });

    let mut objects = vec![player];

    let mut game = Game {
        map: map::make_map(&mut objects),
        messages: Messages::new(),
        inventory: vec![],
        dungeon_level: 1,
        spells: vec![
            Spells::Heal,
            Spells::Lightning,
            Spells::Freeze,
            Spells::Fireball,
        ],
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

        level_up(tcod, game, objects);

        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(tcod, game, objects);
        if player_action == PlayerAction::Exit {
            if objects[PLAYER].alive {
                save_game(game, objects).unwrap();
            }
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

pub fn next_level(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    game.messages
        .add("You take a moment to rest and recover.", VIOLET);

    let heal_hp = objects[PLAYER].fighter.map_or(0, |f| f.max_hp / 2);
    objects[PLAYER].heal(heal_hp);
    let recover_mana = objects[PLAYER].fighter.map_or(0, |f| f.max_mana / 2);
    objects[PLAYER].recover_mana(recover_mana);

    game.messages.add(
        "After a rare moment of peace, you descend deeper into \
        the heart of the dungeon...",
        RED,
    );

    game.dungeon_level += 1;
    game.map = make_map(objects);
    initialise_fov(tcod, &game.map);
}

pub fn level_up(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
    let player = &mut objects[PLAYER];
    let level_up_exp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
    if player.fighter.map_or(0, |f| f.xp) >= level_up_exp {
        player.level += 1;
        game.messages.add(
            format!(
                "Your battle skills grow stronger! You reached level {}!",
                player.level
            ),
            YELLOW,
        );

        let fighter = objects[PLAYER].fighter.as_mut().unwrap();
        let mut choice = None;
        while choice.is_none() {
            choice = menu::menu(
                "Level up! Choose a buff:\n",
                &vec![
                    format!("+20 HP, from {}", fighter.max_hp),
                    format!("+20 MP, from {}", fighter.max_mana),
                ],
                LEVEL_SCREEN_WIDTH,
                &mut tcod.root,
            );
        }
        fighter.xp -= level_up_exp;
        match choice.unwrap() {
            0 => {
                fighter.max_hp += 20;
                fighter.hp += 20;
            }
            1 => {
                fighter.max_mana += 20;
                fighter.mana += 20;
            }
            _ => unreachable!(),
        }
    }
}
