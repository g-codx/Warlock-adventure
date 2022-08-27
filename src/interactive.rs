use crate::egui::Shape::Vec;
use crate::GameState;

use crate::KeyCode::*;
use crate::prelude::*;
use crate::combat::{AttackButton,HeroSpellButton,AttackDice,ManaDice,Selected};
use crate::player::{Backpack, BagExit, Card, MoveDice, Next, SkillPack, Item, CombatDeckSpell, CardView, AcceptRewardButton, SkillPackExit};

pub struct InteractivePlugin;

impl Plugin for InteractivePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(cursor_position)
            .init_resource::<CursorState>()
            .init_resource::<Interactive>()
            .add_system_set(
                SystemSet::on_update(Combat)
                    .with_system(combat_skill_card_event)
                    .with_system(attack_dice)
                    .with_system(mana_dice)
                    .with_system(attack_button)
                    .with_system(hero_spell_button)
                    .with_system(combat_end_button)
            )
            .add_system_set(
                SystemSet::on_update(World)
                    .with_system(move_dice_button)
                    .with_system(back_pack_button)
                    .with_system(skill_pack_button)
                    .with_system(next_button)
                    .with_system(accept_reward_button)
            )
            .add_system_set(
                SystemSet::on_update(BagPack)
                    .with_system(item)
                    .with_system(back_pack_exit_button)
            )
            .add_system_set(
                SystemSet::on_update(Deck)
                    .with_system(spell)
                    .with_system(skill_pack_exit_button)
            );
    }
}


#[derive(Component, Inspectable, Default)]
pub struct Interactive;


#[derive(Default)]
pub struct CursorState {
    pub world_position: Vec2,
    pub last_left_click: Vec2,
    pub last_left_click_float: Vec2,
    pub last_right_click: Vec2,
}

pub fn cursor_position(
    mut camera_query: Query<(&mut Camera, &GlobalTransform), (With<Camera>)>,
    windows: Res<Windows>,
    mut cursor_state: ResMut<CursorState>,
    buttons: Res<Input<MouseButton>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.get_primary().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        // cursor_state.world_position = world_pos;
        cursor_state.world_position = world_pos.round();

        if buttons.just_pressed(MouseButton::Left) {
            cursor_state.last_left_click_float = world_pos;
            cursor_state.last_left_click = world_pos.round();
            // println!("Last left rounding: {}", cursor_state.last_left_click);
            // println!("Last left rounding: {}", cursor_state.last_left_click_float);
        }

        if buttons.just_pressed(MouseButton::Right) {
            // cursor_state.last_right_click = world_pos;
            cursor_state.last_right_click = world_pos.round();
            // println!("Last left rounding: {}", cursor_state.last_right_click);
        }
    }
}

fn attack_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &AttackButton), (With<Interactive>, With<AttackButton>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<AttackButton>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn hero_spell_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &HeroSpellButton), (With<Interactive>, With<HeroSpellButton>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<HeroSpellButton>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn combat_end_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &CombatEndButton), (With<Interactive>, With<CombatEndButton>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<CombatEndButton>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn attack_dice(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &AttackDice), (With<Interactive>, With<AttackDice>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<AttackDice>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn mana_dice(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &ManaDice), (With<Interactive>, With<ManaDice>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<ManaDice>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn move_dice_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &MoveDice), (With<Interactive>, With<MoveDice>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<MoveDice>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn back_pack_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &Backpack), (With<Interactive>, With<Backpack>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<Backpack>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn back_pack_exit_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &BagExit), (With<Interactive>, With<BagExit>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<BagExit>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn skill_pack_exit_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &SkillPackExit), (With<Interactive>, With<SkillPackExit>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<SkillPackExit>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn accept_reward_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &AcceptRewardButton), (With<Interactive>, With<AcceptRewardButton>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<AcceptRewardButton>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn skill_pack_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &SkillPack), (With<Interactive>, With<SkillPack>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<SkillPack>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn next_button(
    cursor_state: Res<CursorState>,
    mut item_position:
    Query<(&Transform, &mut TextureAtlasSprite, &Next), (With<Interactive>, With<Next>)>,
    buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<Next>>,
) {
    combat_element_event(cursor_state, item_position, WHITE, buttons, selected_query);
}

fn spell(
    cursor_state: Res<CursorState>,
    mut item_position: Query<(&Transform, &mut TextureAtlasSprite, &mut Selected), (With<CardView>, With<Interactive>)>,
    buttons: Res<Input<MouseButton>>,
) {
    bag_element(cursor_state, item_position, buttons);
}

fn item(
    cursor_state: Res<CursorState>,
    mut item_position: Query<(&Transform, &mut TextureAtlasSprite, &mut Selected), (With<Item>, With<Interactive>)>,
    buttons: Res<Input<MouseButton>>,
) {
    bag_element(cursor_state, item_position, buttons);
}

fn bag_element<T: Component>(
    cursor_state: Res<CursorState>,
    mut item_position: Query<(&Transform, &mut TextureAtlasSprite, &mut Selected), (With<T>, With<Interactive>)>,
    buttons: Res<Input<MouseButton>>,
) {
    for (transform, mut sprite, mut selected) in item_position.iter_mut() {
        let item_pos = Vec2::new(
            transform.translation.x.round(), transform.translation.y.round());
        let cursor = cursor_state.world_position;

        if cursor == item_pos {
            sprite.color = HOVER_COLOR;

            if buttons.just_pressed(MouseButton::Left) {
                selected.selected = true;
            } else {
                selected.selected = false;
            }
        } else {
            sprite.color = Color::WHITE;
        }
    }
}




fn combat_skill_card_event(
    cursor_state: Res<CursorState>,
    mut item_position: Query<(&Transform, &mut Sprite, &mut Selected), (With<Card>, With<Interactive>)>,
    buttons: Res<Input<MouseButton>>,
) {
    for (transform, mut sprite, mut selected) in item_position.iter_mut() {
        let item_pos = Vec2::new(
            transform.translation.x.round(), transform.translation.y.round());
        let cursor = cursor_state.world_position;
        let size = sprite.custom_size.unwrap();

        if size.x > 1. && size.y > 1. {
            let range = rectangle_click_range(&item_pos, &size);

            if range.contains(&cursor) {
                sprite.color = HOVER_COLOR;

                if buttons.just_pressed(MouseButton::Left) {
                    selected.selected = true;
                } else {
                    selected.selected = false;
                }

            } else {
                sprite.color = WHITE;
            }
        }
    }
}

//small element 1x1
fn combat_element_event<T: Component>(
    cursor_state: Res<CursorState>,
    mut item_position: Query<(&Transform, &mut TextureAtlasSprite, &T), (With<Interactive>, With<T>)>,
    base_element_color: Color,
    mut buttons: ResMut<Input<MouseButton>>,
    mut selected_query: Query<&mut Selected, With<T>>,
) {
    for (transform, mut sprite, _element) in item_position.iter_mut() {
        let item_pos = Vec2::new(
            transform.translation.x.round(), transform.translation.y.round(),
        );
        let cursor = cursor_state.world_position;
        let mut selected = selected_query.single_mut();

        if cursor == item_pos {
            sprite.color = HOVER_COLOR;

            if buttons.just_pressed(MouseButton::Left) {
                selected.selected = true;
            } else {
                selected.selected = false;
            }
        } else {
            sprite.color = base_element_color;
            selected.selected = false
        }
    }
}

fn rectangle_click_range(item_pos: &Vec2, size: &Vec2) -> std::vec::Vec<Vec2> {
    let mut range = std::vec::Vec::new();

    //x rectangle size and step count
    let x_delta = (size.x / 2.0).round() as usize;

    //y rectangle size and step count
    let y_delta = (size.y / 2.0).round() as usize;

    //centre rectangle with round
    let mut x = item_pos.x;
    let mut y = item_pos.y;

    for _ in 0..=x_delta {
        for step in 0..=y_delta {
            range.push(Vec2::new(x.round() as f32, y.round() as f32));
            if step % 2 == 0 { y = item_pos.y + 1.; } else { y = item_pos.y - 1.; }
        }
        y = item_pos.y;
        x += 1.
    }

    range
}
