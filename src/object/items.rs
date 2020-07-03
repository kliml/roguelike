use tcod::colors::*;
use crate::{Tcod, Game, Object, object, UseResult, PLAYER, closest_monster};

pub fn use_item(inventory_id: usize, tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) {
  use object::Item::*;

  if let Some(item) = game.inventory[inventory_id].item {
      let on_use = match item {
          Heal => cast_heal,
          Lightning => cast_lightning,
      };
      match on_use(inventory_id, tcod, game, objects) {
          UseResult::UsedUp => {
              game.inventory.remove(inventory_id);
          }
          UseResult::Cancelled => {
              game.messages.add("Cancelled", WHITE);
          }
      }
  } else {
      game.messages.add(
          format!("The {} cannot be used.", game.inventory[inventory_id].name),
          WHITE,
      );
  }
}

const HEAL_AMOUNT: i32 = 4;
pub fn cast_heal(
  _inventory_id: usize,
  _tcod: &mut Tcod,
  game: &mut Game,
  objects: &mut Vec<Object>,
) -> UseResult {
  if let Some(fighter) = objects[PLAYER].fighter {
      if fighter.hp == fighter.max_hp {
          game.messages.add("You are already at full health.", RED);
          return UseResult::Cancelled;
      }
      game.messages
          .add("Your wounds start to feel better!", LIGHT_VIOLET);
      objects[PLAYER].heal(HEAL_AMOUNT);
      return UseResult::UsedUp;
  }
  UseResult::Cancelled
}

const LIGHTNING_DAMAGE: i32 = 40;
const LIGHTNING_RANGE: i32 = 5;
pub fn cast_lightning(
  _inventory_id: usize,
  tcod: &mut Tcod,
  game: &mut Game,
  objects: &mut Vec<Object>,
) -> UseResult {
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
      objects[monster_id].take_damage(LIGHTNING_DAMAGE, game);
      UseResult::UsedUp
  } else {
      game.messages.add("No enemy is close enough to strike", RED);
      UseResult::Cancelled
  }
}
