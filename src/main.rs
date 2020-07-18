use tcod::console::*;
use tcod::input::{Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};
mod ai;
mod engine;
mod game;
mod help;
mod map;
mod object;
mod renderer;
mod settings;

use help::closest_monster;
use help::get_names_under_mouse;
use map::Map;
use object::Object;

use game::*;
use settings::*;

// FPS Limit
const LIMIT_FPS: i32 = 20;

// FOV
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

pub struct Tcod {
    root: Root,
    con: Offscreen,
    panel: Offscreen,
    fov: FovMap,
    key: Key,
    mouse: Mouse,
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("res/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("wizard dungeon")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
        fov: FovMap::new(map::MAP_WIDTH, map::MAP_HEIGHT),
        key: Default::default(),
        mouse: Default::default(),
    };

    menu::main_menu(&mut tcod);
}
