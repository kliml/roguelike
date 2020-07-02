use std::cmp;

use super::object::Object;
use crate::Tcod;
use crate::PLAYER;
use crate::game::Game;

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut Vec<T>) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

pub fn closest_monster(tcod: &Tcod, objects: &Vec<Object>, max_range: i32) -> Option<usize> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32;

    for (id, object) in objects.iter().enumerate() {
        if (id != PLAYER)
            && object.fighter.is_some()
            && object.ai.is_some()
            && tcod.fov.is_in_fov(object.x, object.y)
        {
            let dist = objects[PLAYER].distance_to(object);
            if dist < closest_dist {
                closest_enemy = Some(id);
                closest_dist = dist;
            }
        }
    }
    closest_enemy
}

pub fn target_tile(
    tcod: &mut Tcod,
    game: &mut Game,
    objects: &Vec<Object>,
    max_range: Option<f32>,
) -> Option<(i32, i32)> {
    use tcod::input::{ self, Event, KeyCode::Escape };
    use crate::render_all;

    loop {
        tcod.root.flush();
        let event = input::check_for_event(input::KEY_PRESS | input::MOUSE).map(|e| e.1);
        match event {
            Some(Event::Mouse(m)) => tcod.mouse = m,
            Some(Event::Key(k)) => tcod.key = k,
            None => tcod.key = Default::default(),
        }
        render_all(tcod, game, objects, false);

        let (x, y) = (tcod.mouse.cx as i32, tcod.mouse.cy as i32);

        let in_fov = tcod.fov.is_in_fov(x, y);
        let in_range = max_range.map_or(true, |r| objects[PLAYER].distance(x, y) <= r);
        if tcod.mouse.lbutton_pressed && in_fov && in_range {
            return Some((x, y));
        }
        if tcod.mouse.rbutton_pressed || tcod.key.code == Escape {
            return None;
        }
    }
}
