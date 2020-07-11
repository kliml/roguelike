use crate::*;
use tcod::colors::*;
use crate::settings::*;

pub fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &Vec<Object>, fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[PLAYER];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
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
                tcod.con
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| {
            tcod.fov.is_in_fov(o.x, o.y)
                || o.always_visible && game.map[o.x as usize][o.y as usize].explored
        })
        .collect();
    to_draw.sort_by(|o1, o2| o1.blocks.cmp(&o2.blocks));
    for object in &to_draw {
        object.draw(&mut tcod.con);
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

    // Prepare for rendering
    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();

    // Show player stats
    let hp = objects[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER].fighter.map_or(0, |f| f.max_hp);
    render_bar(
        &mut tcod.panel,
        1,
        1,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        LIGHT_RED,
        DARK_RED,
    );
    let mana = objects[PLAYER].fighter.map_or(0, |f| f.mana);
    let max_mana = objects[PLAYER].fighter.map_or(0, |f| f.max_mana);
    render_bar(
        &mut tcod.panel,
        1,
        2,
        BAR_WIDTH,
        "MP",
        mana,
        max_mana,
        LIGHT_BLUE,
        DARK_BLUE,
    );
    let xp = objects[PLAYER].fighter.map_or(0, |f| f.xp);
    let xp_needed = LEVEL_UP_BASE + objects[PLAYER].level * LEVEL_UP_FACTOR;
    render_bar(
        &mut tcod.panel,
        1,
        3,
        BAR_WIDTH,
        "EXP",
        xp,
        xp_needed,
        LIGHT_GREEN,
        DARK_GREEN,
    );

    // Display dungeon level
    tcod.panel.print_ex(
        1,
        4,
        BackgroundFlag::None,
        TextAlignment::Left,
        format!("Dungeon level: {}", game.dungeon_level),
    );

    // Display object names under mouse
    tcod.panel.set_default_foreground(LIGHT_GREY);
    tcod.panel.print_ex(
        1,
        0,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(tcod.mouse, objects, &tcod.fov),
    );

    // Display messages
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // Blit panel to root
    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_Y),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    )
}

fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;
    panel.set_default_background(back_color);
    panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }
    panel.set_default_foreground(WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, maximum),
    );
}
