mod systems;
mod components;

pub use crate::prelude::*;
pub use crate::world::systems::*;
pub use crate::world::components::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PreStartup, Self::init_item_pull,
            )
            .add_event::<UpdateEvent>()
            .add_event::<EncounterEvent>()
            .add_system_set(
                SystemSet::on_enter(World)
                    .with_system(show_buttons)
                    .with_system(spawn_player_world_stats)
            )
            .add_system_set(
                SystemSet::on_exit(World)
                    .with_system(hide_buttons)
                    .with_system(despawn_world_player_stats_bar)
            )
            .add_system_set(
                SystemSet::on_update(World)
                    .with_system(world_object_event.after(player_movement))
                    .with_system(accept_reward.after(world_object_event))
                    .with_system(move_dice)
                    .with_system(bag_button)
                    .with_system(skill_pack_button)
                    .with_system(town_event)
                    .with_system(next_day_button)
                    .with_system(update_move_points_text)
                    .with_system(update_days_count_text)
                    .with_system(update_world_attack_text)
                    .with_system(update_world_defense_text)
                    .with_system(update_world_health_text)
                    .with_system(update_world_mana_text)
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
                    .with_system(update_stats)
            )
            .add_system_set(
                SystemSet::on_enter(Deck)
                    .with_system(spawn_skill_pack_interface)
                    .with_system(bag_interface_camera)
            )
            .add_system_set(
                SystemSet::on_update(Deck)
                    .with_system(skill_pack_button_exit)
                    .with_system(take_spell)
                    .with_system(remove_spell)
                    .with_system(update_spell_position)
            )
            .add_system_set(
                SystemSet::on_exit(Deck)
                    .with_system(despawn_bag_interface)
            )
            .add_startup_system(spawn_world_interface);
    }
}

impl WorldPlugin {
    fn init_item_pull(commands: Commands) {
        ItemPull::create_item_pull(commands);
    }
}
