use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use crate::prelude::*;
use crate::combat::{CombatManager,CombatStats,Enemy};
use crate::player::{Card, EncounterTracker, Item, Player};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_type::<EncounterTracker>()
            .register_type::<FrameAnimation>()
            .register_type::<CombatManager>()
            .register_inspectable::<CombatStats>()
            .register_inspectable::<Player>()
            .register_inspectable::<Interactive>()
            .register_inspectable::<Enemy>()
            .register_inspectable::<Card>()
            .register_inspectable::<Tile>()
            .register_inspectable::<Item>()
        ;
    }
}