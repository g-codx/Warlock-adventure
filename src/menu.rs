use bevy::app::AppExit;
use crate::KeyCode::Escape;
use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(esc)
            .add_system_set(
            SystemSet::on_enter(Menu)
                .with_system(spawn_menu)
                .with_system(hide_buttons)
                .with_system(despawn_world_player_stats_bar)
                .with_system(hide_map)
                .with_system(hide_player)
            )
            .add_system_set(
                SystemSet::on_update(Menu)
                    .with_system(start_button)
                    .with_system(exit_button)
            )
            .add_system_set(
                SystemSet::on_exit(Menu)
                    .with_system(despawn_menu)
            );
    }

}

pub fn esc(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if state.current() == &Menu {
        return;
    }
    if keys.just_pressed(Escape) {
        state.set(Menu).unwrap();
    }
}

#[derive(Component)]
pub struct ButtonActive(pub bool);

pub struct UiAssets {
    pub font: Handle<Font>,
    pub button: Handle<Image>,
    pub button_pressed: Handle<Image>,
}

#[derive(Component)]
pub struct UiCameraMarker;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ExitButton;

enum ButtonAction {
    Start,
    Exit
}

fn despawn_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<UiCameraMarker>>,
    background_query: Query<Entity, With<Battleground>>
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


pub fn spawn_menu(mut commands: Commands, texture_storage: Res<TextureStorage>) {
    commands.spawn_bundle(
        UiCameraBundle {
            transform: Transform::from_xyz(0., 0., 1000.),
            ..default()
        })
        .insert(UiCameraMarker);

    spawn_menu_button(
        &texture_storage,
        &mut commands,
        StartButton,
        String::from("Start game"),
        Rect {
            left: Val::Percent(25.),
            ..default()
        }
    );
    spawn_menu_button(
        &texture_storage,
        &mut commands,
        ExitButton,
        String::from("Exit game"),
        Rect {
            right: Val::Percent(25.),
            top: Val::Percent(6.),
            ..default()
        }
    );
    spawn_combat_battleground(
        &mut commands,
        &texture_storage,
        &EnemyType::Demon,
        Transform::from_xyz(10., -10., 800.)
    );
}

fn start_button(
    state: ResMut<State<GameState>>,
    exit: EventWriter<AppExit>,
    interaction_query:
    Query<(&Children, &mut ButtonActive, &Interaction), (With<StartButton>, Changed<Interaction>)>,
    image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
) {
    button_listener(
        state,
        exit,
        interaction_query,
        image_query,
        ui_assets,
        ButtonAction::Start
    );
}

fn exit_button(
    state: ResMut<State<GameState>>,
    exit: EventWriter<AppExit>,
    interaction_query:
    Query<(&Children, &mut ButtonActive, &Interaction), (With<ExitButton>, Changed<Interaction>)>,
    image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
) {
    button_listener(
        state,
        exit,
        interaction_query,
        image_query,
        ui_assets,
        ButtonAction::Exit
    );
}

fn button_listener<T: Component>(
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query:
    Query<(&Children, &mut ButtonActive, &Interaction), (With<T>, Changed<Interaction>)>,
    mut image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
    button_action: ButtonAction
) {

    for (children, mut active, interaction) in interaction_query.iter_mut() {

        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Clicked => {
                if active.0 {
                    image.0 = ui_assets.button_pressed.clone();
                    active.0 = false;
                }
                match button_action {
                    ButtonAction::Start => {
                        state.set(World).unwrap();
                    },
                    ButtonAction::Exit => {
                        exit.send(AppExit);
                    }
                }
            }
            Interaction::Hovered | Interaction::None => {
                image.0 = ui_assets.button.clone();

            }
        }
    }
}
