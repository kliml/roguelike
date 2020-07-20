use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::cmp;
use tcod::colors::*;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::help::mut_two;
use crate::object::*;
use crate::settings::*;

use items::Item;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
pub const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
pub const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

// Monster limit
const MAX_ROOM_MONSTERS: i32 = 3;

// Items limit
const MAX_ROOM_ITEMS: i32 = 2;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub blocked: bool,
    pub explored: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            explored: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            explored: false,
            block_sight: true,
        }
    }
}

pub type Map = Vec<Vec<Tile>>;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x1)
            && (self.x2 >= other.x2)
            && (self.y1 <= other.y1)
            && (self.y2 >= other.y2)
    }
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn make_map(objects: &mut Vec<Object>) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Clear all objects except PLAYER
    objects.truncate(1);

    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            create_room(new_room, &mut map);

            place_objects(new_room, &map, objects);

            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // Starting room
                objects[PLAYER].set_pos(new_x, new_y);
            } else {
                // All other rooms

                // Decide which way to connect rooms
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                } else {
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                }
            }

            rooms.push(new_room);
        }
    }

    let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
    let mut stairs = Object::new(last_room_x, last_room_y, '<', WHITE, "stairs", false);
    stairs.always_visible = true;
    objects.push(stairs);

    map
}

fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_blocked(x, y, map, objects) {
            let monster = if rand::random::<f32>() < 0.8 {
                let mut ork = Object::new(x, y, 'o', DESATURATED_GREEN, "ork", true);
                ork.alive = true;
                ork.fighter = Some(Fighter {
                    max_hp: 10,
                    hp: 10,
                    max_mana: 0,
                    mana: 0,
                    defense: 0,
                    power: 3,
                    xp: 35,
                    on_death: DeathCallback::Monster,
                });
                ork.ai = Some(Ai::Basic);
                ork
            } else {
                let mut troll = Object::new(x, y, 'T', DARKER_GREEN, "troll", true);
                troll.alive = true;
                troll.fighter = Some(Fighter {
                    max_hp: 16,
                    hp: 16,
                    max_mana: 0,
                    mana: 0,
                    defense: 1,
                    power: 4,
                    xp: 100,
                    on_death: DeathCallback::Monster,
                });
                troll.ai = Some(Ai::Basic);
                troll
            };
            objects.push(monster);
        }
    }

    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);

    let items = [(Item::Heal, 4), (Item::Mana, 4), (Item::Vision, 1)];
    let items_dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
    let mut rng = thread_rng();

    for _ in 0..num_items {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_blocked(x, y, map, objects) {
            let mut item = match items[items_dist.sample(&mut rng)].0 {
                Item::Heal => {
                    let mut object = Object::new(x, y, '!', VIOLET, "healing potion", false);
                    object.item = Some(Item::Heal);
                    object
                }
                Item::Mana => {
                    let mut object = Object::new(x, y, '!', LIGHT_BLUE, "mana potion", false);
                    object.item = Some(Item::Mana);
                    object
                }
                Item::Vision => {
                    let mut object = Object::new(x, y, '$', PINK, "vision scroll", false);
                    object.item = Some(Item::Vision);
                    object
                }
                _ => unreachable!(),
            };
            item.always_visible = true;
            objects.push(item);
        }
    }
}

fn is_blocked(x: i32, y: i32, map: &Map, objects: &Vec<Object>) -> bool {
    if map[x as usize][y as usize].blocked {
        return true;
    }

    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
}

pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut Vec<Object>) {
    let (x, y) = objects[id].pos();

    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut Vec<Object>) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER, target_id, objects);
            player.attack(target, game);
        }

        None => {
            move_by(PLAYER, dx, dy, &game.map, objects);
        }
    }
}
