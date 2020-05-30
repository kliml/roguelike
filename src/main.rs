use tcod::colors::*;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

mod misc;

// Window size
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

use misc::ai;
use misc::map;
use misc::map::Map;
use misc::object::Object;
use misc::object::Fighter;
use misc::object::Ai;

// FPS Limit
const LIMIT_FPS: i32 = 20;

// Player pos in vec
pub const PLAYER: usize = 0;

// FOV
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

pub struct Game {
    map: Map,
}

pub struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

use PlayerAction::*;

fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &Vec<Object>, fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[PLAYER];
        tcod.fov.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }
    
    for x in 0..map::MAP_WIDTH {
        for y in 0..map::MAP_HEIGHT {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                // Outside of FOV
                (false, true) => map::COLOR_DARK_WALL,
                (false, false) => map::COLOR_DARK_GROUND,
                // Inside FOV
                (true, true) => map::COLOR_LIGHT_WALL,
                (true, false) => map::COLOR_LIGHT_GROUND,
            };
            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }
            if *explored {
                tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
    
    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        } else {
            //tcod.con.set_char_foreground(object.x, object.y, BLACK);

        }
    }
    
    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

    tcod.root.set_default_background(WHITE);
    if let Some(fighter) = objects[PLAYER].fighter {
        tcod.root.print_ex(
            1,
            SCREEN_HEIGHT - 2,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("HP: {}/{}",fighter.hp, fighter.max_hp)
        );
    }
}



fn handle_keys(tcod: &mut Tcod, game: &Game, objects: &mut Vec<Object>) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    
    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].alive;
    match (key, key.text(), player_alive) {
        (Key {
            code: Enter,
            alt: true,
            ..
        }, _, _) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }

        (Key { code: Escape, .. }, _, _) => Exit,
        (Key { code: Up, .. }, _, _) => {
            map::player_move_or_attack(0, -1, game, objects);
            TookTurn
        }
        (Key { code: Down, .. }, _, _) => {
            map::player_move_or_attack(0, 1, game, objects);
            TookTurn
        }
        (Key { code: Left, .. }, _, _) => {
            map::player_move_or_attack(-1, 0, game, objects);
            TookTurn
        }
        (Key { code: Right, .. }, _, _) => {
            map::player_move_or_attack(1, 0, game, objects);
            TookTurn
        }

        _ => DidntTakeTurn,
    }
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);
    
    let root = Root::initializer()
        .font("res/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("roguelike")
        .init();
    
    let mut tcod = Tcod {
        root,
        con: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        fov: FovMap::new(map::MAP_WIDTH, map::MAP_HEIGHT),
    };
    
    // Player
    let mut player = Object::new(0, 0, '@', WHITE, "player", false);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 5,
    });
    
    let mut objects = vec![player];

    let mut game = Game {
        map: map::make_map(&mut objects),
    };

    // Populate FOV map
    for x in 0..map::MAP_WIDTH {
        for y in 0..map::MAP_HEIGHT {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }

    let mut previous_player_position = (-1, -1);
    
    while !tcod.root.window_closed() {
        tcod.con.clear();
        
        for object in &objects {
            object.draw(&mut tcod.con);
        }

        // render
        let fov_recompute = previous_player_position != (objects[PLAYER].pos());
        render_all(&mut tcod, &mut game, &objects, fov_recompute);

        tcod.root.flush();

        let player = &mut objects[PLAYER];
        previous_player_position = (player.x, player.y);
        let player_action = handle_keys(&mut tcod, &game, &mut objects);
        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai::ai_take_turn(id, &tcod, &game, &mut objects);
                }
            }
        }
    }
}
