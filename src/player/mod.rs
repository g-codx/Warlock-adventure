mod components;
mod systems;

pub use crate::player::systems::*;
pub use crate::player::components::*;
use crate::prelude::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(World)
                    .with_system(show_player)
            )
            .add_system_set(
                SystemSet::on_update(World)
                    .with_system(player_encounter_checking.after(player_movement))
                    .with_system(player_movement.after(cursor_position))
            )
            .add_system_set(
                SystemSet::on_exit(World)
                    .with_system(hide_player)
            )
            .add_startup_system(spawn_player);
    }
}
