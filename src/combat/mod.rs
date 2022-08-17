mod components;
mod systems;

use crate::prelude::*;
pub use crate::combat::components::*;
pub use crate::combat::components::CombatState::{EnemyAttack, EnemyTurn, Finalize, PlayerAttack, PlayerTurn, PreState};
pub use crate::GameState;
pub use crate::combat::systems::*;
use crate::CombatState::End;


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(PreState)
            .add_event::<FightEvent>()
            .init_resource::<CombatManager>()
            .add_system_set(
                SystemSet::on_update(EnemyTurn)
                    .with_system(enemy_turn)
            )
            .add_system_set(
                SystemSet::on_update(PlayerTurn)
                    .with_system(attack_dice_roll)
                    .with_system(mana_dice_roll)
                    .with_system(attack_button)
                    .with_system(skip_button)
                    .with_system(use_card)
                    .with_system(update_damage_text.after(use_card))
                    .with_system(update_mana_poll_text.after(use_card))
                    .with_system(update_defense_text.after(use_card))
                    .with_system(update_health_text.after(use_card))
            )
            .add_system_set(
                SystemSet::on_enter(Finalize)
                    .with_system(update_enemy_health_text.before(finalize))
                    .with_system(update_health_text.before(finalize))
                    .with_system(finalize)
                    .with_system(update_attack_dice_sprite.after(finalize))
                    .with_system(update_mana_dice_sprite.after(finalize))
                    .with_system(update_round_text.after(finalize))
            )
            .add_system_set(
                SystemSet::on_update(Combat)
                    .with_system(combat_camera)
                    .with_system(damage_calculation)
                    .with_system(enemy_attack_effect)
            )
            .add_system_set(
                SystemSet::on_enter(Combat)
                    .with_system(init_manager)
                    .with_system(set_starting_state)
                    .with_system(spawn_enemy)
                    .with_system(spawn_interface)
            )
            .add_system_set(
                SystemSet::on_enter(End)
                    .with_system(end_combat)
            )
            .add_system_set(
                SystemSet::on_update(End)
                    .with_system(combat_end_button)
            )
            .add_system_set(
                SystemSet::on_exit(Combat)
                    .with_system(despawn_enemy)
                    .with_system(despawn_player_text)
                    .with_system(despawn_bottom_items)
                    .with_system(despawn_top_items)
                    .with_system(despawn_battleground)
                    .with_system(manager_default)
            );
    }
}




