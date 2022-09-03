use bevy::sprite::collide_aabb::collide;
use crate::prelude::*;
use crate::player::components::*;

pub fn player_movement(
    mut player: Query<(&Player, &mut Transform), With<Player>>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    mut cursor_state: ResMut<CursorState>,
    buttons: Res<Input<MouseButton>>,
    mut move_points_query: Query<&mut MoveDice>,
) {
    let (pl, mut transform) = player.single_mut();
    let mut move_points = move_points_query.single_mut();

    if buttons.just_pressed(MouseButton::Right) {
        transform.translation.x = cursor_state.last_right_click.x;
        transform.translation.y = cursor_state.last_right_click.y;
    }


    // if buttons.just_pressed(MouseButton::Right) && move_points.value != 0 {
    //     let x_target = cursor_state.last_right_click.x;
    //     let y_target = cursor_state.last_right_click.y;
    //
    //     println!("{}", x_target);
    //     println!("{}", y_target);
    //
    //     let x_ = (transform.translation.x - x_target).abs();
    //     let y_ = (transform.translation.y - y_target).abs();
    //
    //     if x_ <= 1. && y_ <= 1. {
    //         let target = Vec3::new(x_target, y_target, 100.);
    //
    //         if !wall_query
    //             .iter()
    //             .any(|&transform|
    //                 collide_check(target, transform.translation))
    //         {
    //             move_points.value = std::cmp::max(
    //                 move_points.value - 1,
    //                 0,
    //             );
    //
    //             transform.translation.x = cursor_state.last_right_click.x;
    //             transform.translation.y = cursor_state.last_right_click.y;
    //
    //             println!("{:?}", move_points.value);
    //         }
    //     }
    // }
}

pub fn player_encounter_checking(
    player_query: Query<&Transform, With<Player>>,
    encounter_query: Query<(&Transform, &EncounterType), (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
    mut encounter_event: EventWriter<EncounterEvent>,
) {
    let player_translation = player_query.single().translation;

    for (transform, enc_type) in encounter_query.iter() {
        if collide_check(transform.translation, player_translation) && !enc_type.1 {
            encounter_event.send(EncounterEvent(enc_type.0));
            state.set(Combat).expect("Failed to change states");
        }
    }
}

pub fn collide_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(1.0),
        wall_translation,
        Vec2::splat(1.0),
    );
    collision.is_some()
}

pub fn spawn_player(mut commands: Commands, frame_sheet: Res<FramesSheet>) {
    let sprite = TextureAtlasSprite {
        index: 0,
        color: Color::rgb(1., 1., 1.),
        custom_size: Some(Vec2::new(1., 1.)),
        ..default()
    };

    let animation = FrameAnimation {
        timer: Timer::from_seconds(0.2, true),
        frames: frame_sheet.hero_idle.to_vec(),
        current_frame: 0,
    };

    let item_build = ItemBuild {
        defense: None,
        attack: None,
        mana: None,
        health: None,
    };

    let mut player = Player {
        is_selected: false,
        combat_deck: vec![],
        items_bag: vec![],
        item_build,
        deck: vec![],
    };

    player.add_in_deck(1, 1);
    player.add_in_deck(5, 1);

    let _ = commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: frame_sheet.hero_idle_atlas_handle.clone(),
        transform: Transform {
            translation: Vec3::new(6., -3., 500.),
            ..default()
        },
        ..Default::default()
    })
        .insert(Name::new("Player"))
        .insert(player)
        .insert(CombatStats {
            health: 10,
            defense: 0,
            attack: 0,
            max_health: 10,
            mana: 0,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true)
        })
        .insert(animation)
        .id();
}

pub fn show_player(
    player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    toggle_player_visible(player_query, children_query, child_visibility_query, true);
}

pub fn hide_player(
    player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    toggle_player_visible(player_query, children_query, child_visibility_query, false);
}

fn toggle_player_visible(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    child_visibility_query: Query<&mut Visibility, Without<Player>>,
    show: bool
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = show;
    toggle_visible(children_query, child_visibility_query, show)
}
