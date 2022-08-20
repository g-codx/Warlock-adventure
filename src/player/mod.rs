pub(crate) mod components;
mod systems;

pub use crate::player::systems::*;
pub use crate::player::components::*;
use crate::prelude::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UpdateEvent>()
            .add_event::<EncounterEvent>()
            .add_system_set(
                SystemSet::on_enter(World)
                    .with_system(show_player)
                    .with_system(show_buttons)
            )
            .add_system_set(
                SystemSet::on_exit(World)
                    .with_system(hide_player)
                    .with_system(hide_buttons)
            )
            .add_system_set(
                SystemSet::on_update(World)
                    .with_system(player_encounter_checking.after(player_movement))
                    .with_system(player_world_event_checking.after(player_movement))
                    .with_system(player_movement.after(cursor_position))
                    .with_system(move_dice)
                    .with_system(bag_button)
                    .with_system(skill_pack_button)
            )
            .add_system_set(
                SystemSet::on_enter(BagPack)
                    .with_system(spawn_bag_interface)
                    .with_system(bag_interface_camera)
            )
            .add_system_set(
                SystemSet::on_update(BagPack)
                    .with_system(bag_button_exit.after(take_item))
                    .with_system(take_item)
            )
            .add_system_set(
                SystemSet::on_exit(BagPack)
                    .with_system(despawn_bag_interface)
            )
            .add_system_set(
                SystemSet::on_enter(Deck)
                    .with_system(spawn_skill_pack_interface)
                    .with_system(bag_interface_camera)
            )
            .add_system_set(
                SystemSet::on_update(Deck)
                    .with_system(bag_button_exit)
                    .with_system(take_spell)
                    .with_system(remove_spell)
                    .with_system(update_spell_position)
            )
            .add_system_set(
                SystemSet::on_exit(Deck)
                    .with_system(despawn_bag_interface)
            )
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_world_interface);
    }
}