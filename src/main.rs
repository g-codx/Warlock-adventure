use prelude::*;
use crate::menu::MenuPlugin;

mod player;
mod debug;
mod map;
mod camera;
mod interactive;
mod combat;
mod graphics;
mod template;
mod world;
mod menu;


mod prelude {
    pub extern crate core;
    use std::ops::Range;
    pub use bevy::prelude::*;
    pub use bevy::window::{*, WindowMode::*};
    pub use bevy::render::camera::*;
    pub use bevy::core::*;
    pub use bevy_inspector_egui::*;
    pub use rand::prelude::*;

    pub use crate::debug::*;
    pub use crate::map::*;
    pub use crate::camera::CameraPlugin;
    pub use crate::interactive::*;
    pub use crate::GameState::*;
    pub use crate::combat::*;
    pub use crate::graphics::*;
    pub use crate::template::*;
    pub use crate::world::*;
    pub use crate::player::*;
    pub use crate::menu::*;

    pub const SILVER: Color = Color::rgb(0.75, 0.75, 0.75);
    pub const RGB: f32 = 255.0;
    pub const RESOLUTION: f32 = 16.0 / 9.0;
    pub const SCREEN_HEIGHT: f32 = 1080.;
    pub const CAMERA_SCALE: f32 = 6.0;
    pub const CAMERA_MOVE_BORDER: Range<f32> = 0. .. 25.;
    pub const CAMERA_SPEED: f32 = 7.0;
    pub const TILE_SIZE: f32 = 1.;
    pub const HOVER_COLOR: Color = Color::rgb(0.18, 0.55, 0.34);
    pub const COMBAT_INTERFACE_COLOR: Color = Color::rgb(208. / RGB, 253. / RGB, 255. / RGB);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

    pub type WorldEventFilter = (With<WorldEventMarker>, Without<Player>);
    pub type NonInteractiveItemFilter = (With<NonInteractiveItem>, Without<Selected>, Without<Interactive>);
    pub type SpawnerFilter = (With<EncounterSpawner>, Without<Player>);
    pub type UiCameraFilter = (Without<Player>, Without<UiCameraMarker>, With<Camera>);
    pub type CameraFilter = (With<Camera>, Without<UiCameraMarker>);

}


fn main() {
    App::new()
        .add_state(Menu)
        .insert_resource(ClearColor(SILVER))
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            width: SCREEN_HEIGHT * RESOLUTION,
            height:SCREEN_HEIGHT,
            title: "Warlock".to_string(),
            present_mode: PresentMode::Mailbox,
            resizable: false,
            mode: Fullscreen,
            ..Default::default()
        })
        .add_plugin(InteractivePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(TemplatePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(MenuPlugin)
        .run();
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Menu,
    World,
    Combat,
    BagPack,
    Deck,
}
