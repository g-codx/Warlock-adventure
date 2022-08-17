use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashMap;
use crate::egui::Ui;
use crate::player::components::{Backpack, EncounterTracker, MoveDice, Player, SkillPack};
use crate::player::{Exit, BagInterface, Next, WorldInterface, SkillPackInterface, ItemView, Item, ItemBuild, CardView, CombatDeckSpell, UpdateEvent};
use crate::prelude::*;


pub fn bag_button(
    selected_query: Query<&Selected, With<Backpack>>,
    mut state: ResMut<State<GameState>>,
) {
    let selected = selected_query.single();

    if selected.selected {
        state.set(BagPack).expect("Failed to change states");
    }
}

pub fn skill_pack_button(
    selected_query: Query<&Selected, With<SkillPack>>,
    mut state: ResMut<State<GameState>>,
) {
    let selected = selected_query.single();

    if selected.selected {
        state.set(SillPack).expect("Failed to change states");
    }
}

pub fn bag_button_exit(
    selected_query: Query<&Selected, With<Exit>>,
    mut state: ResMut<State<GameState>>,
) {
    let selected = selected_query.single();
    if selected.selected {
        state.set(World).expect("Failed to change states");
    }
}

pub fn take_spell(
    mut selected_query: Query<(&mut CardView, &Selected), (Without<CombatDeckSpell>)>,
    mut player_query: Query<&mut Player>,
    mut update_event: EventWriter<UpdateEvent>,
) {
    let mut player = player_query.single_mut();
    for (mut item, selected) in selected_query.iter_mut() {
        if selected.selected && player.can_add(item.level) {
            println!("added");
            player.add_spell(item.clone());
            update_event.send(UpdateEvent(true));
        }
    }
}

pub fn remove_spell(
    mut selected_query: Query<(&mut CardView, &Selected),(With<CombatDeckSpell>)>,
    mut player_query: Query<&mut Player>,
    mut update_event: EventWriter<UpdateEvent>,
) {
    let mut player = player_query.single_mut();
    for (mut item, selected) in selected_query.iter_mut() {
        if selected.selected {
            println!("deleted");
            player.remove(item.id);
            update_event.send(UpdateEvent(true));
        }
    }
}

pub fn update_spell_position(
    mut commands: Commands,
    texture_storage: Res<TextureStorage>,
    mut player_query: Query<&mut Player>,
    template_storage: Res<TemplateStorage>,
    mut update_event: EventReader<UpdateEvent>,
    mut bag_interface_query: Query<Entity, With<BagInterface>>,
    combat_deck_spells_query: Query<Entity, With<CombatDeckSpell>>
) {
    let mut sprites = Vec::with_capacity(5);
    let mut player = player_query.single_mut();
    let mut combat_cards = &mut player.combat_deck;
    let mut interface = bag_interface_query.single_mut();


    if let Some(update_event) = update_event.iter().next() {
        if update_event.0 {
            for (entity) in combat_deck_spells_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            let skill_build_positions = vec![
                Transform::from_xyz(3.5, -4.5, 700.),
                Transform::from_xyz(3.5, -6., 700.),
                Transform::from_xyz(3.5, -7.5, 700.),
                Transform::from_xyz(5., -4.5, 700.),
                Transform::from_xyz(5., -6., 700.),
                Transform::from_xyz(6.5, -4.5, 700.),
            ];

            combat_cards.sort_by(|a, b| a.level.cmp(&b.level));

            combat_cards
                .iter()
                .zip(skill_build_positions)
                .for_each(|(c, p)| {
                    sprites.push(
                        spawn_spell_in_bag(
                            &mut commands,
                            &texture_storage,
                            &p,
                            &template_storage,
                            c.id,
                            true
                        )
                    )
                });


            commands.entity(interface).push_children(&sprites);
        }
    }
}


pub fn take_item(
    mut selected_query: Query<(&mut Item, &Selected, Entity)>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    texture_storage: Res<TextureStorage>,
    template_storage: Res<TemplateStorage>,
    bag_interface_query: Query<Entity, With<BagInterface>>,
) {
    let mut player = player_query.single_mut();
    let bag_interface = bag_interface_query.single();
    for (mut item, selected, entity) in selected_query.iter_mut() {
        if selected.selected {
            let itm = match item.buff_type {
                CardAction::DefenceBuff => {
                    player.item_build.defense = Some((item.id, item.value));

                    spawn_bag_item(
                        &mut commands,
                        &texture_storage,
                        Transform::from_xyz(5., -6.5, 700.),
                        &template_storage,
                        item.id,
                        false,
                    )
                }
                CardAction::AttackBuff => {
                    player.item_build.attack = Some((item.id, item.value));

                    spawn_bag_item(
                        &mut commands,
                        &texture_storage,
                        Transform::from_xyz(6., -6.5, 700.),
                        &template_storage,
                        item.id,
                        false,
                    )
                }
                CardAction::ManaBuff => {
                    player.item_build.mana = Some((item.id, item.value));

                    spawn_bag_item(
                        &mut commands,
                        &texture_storage,
                        Transform::from_xyz(4., -6.5, 700.),
                        &template_storage,
                        item.id,
                        false,
                    )
                }
                CardAction::HealthBuff => {
                    player.item_build.health = Some((item.id, item.value));

                    spawn_bag_item(
                        &mut commands,
                        &texture_storage,
                        Transform::from_xyz(5., -5.5, 700.),
                        &template_storage,
                        item.id,
                        false,
                    )
                }
                _ => panic!("Illegal template state")
            };
            commands.entity(bag_interface).push_children(&[itm]);
            commands.entity(entity).despawn_recursive();

            let index = player.items_bag
                .iter()
                .position(|i| i.id == item.id)
                .unwrap();

            player.items_bag.remove(index);
        }
    }
}

pub fn despawn_bag_interface(
    mut commands: Commands,
    bag_query: Query<Entity, With<BagInterface>>,
) {
    for (entity) in bag_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn bag_interface_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 7.5;
    camera_transform.translation.y = -9.0;
}


pub fn spawn_skill_pack_interface(
    mut commands: Commands,
    texture_storage: Res<TextureStorage>,
    mut player_query: Query<&Player>,
    template_storage: Res<TemplateStorage>,
    mut update_event: EventWriter<UpdateEvent>,
) {
    let mut sprites = Vec::with_capacity(5);
    let player = player_query.single();
    let cards_in_bag = &player.deck;

    let skill_in_bag_positions = vec![
        Transform::from_xyz(3.5, -9., 700.),
        Transform::from_xyz(5., -9., 700.),
        Transform::from_xyz(6.5, -9., 700.),
        Transform::from_xyz(8., -9., 700.),
        Transform::from_xyz(3.5, -10.5, 700.),
        Transform::from_xyz(5., -10.5, 700.),
        Transform::from_xyz(6.5, -10.5, 700.),
        Transform::from_xyz(8., -10.5, 700.),
        Transform::from_xyz(3.5, -12., 700.),
        Transform::from_xyz(5., -12., 700.),
        Transform::from_xyz(6.5, -12., 700.),
        Transform::from_xyz(8., -12., 700.),
        Transform::from_xyz(3.5, -13.5, 700.),
        Transform::from_xyz(5., -13.5, 700.),
        Transform::from_xyz(6.5, -13.5, 700.),
        Transform::from_xyz(8., -13.5, 700.),
    ];


    cards_in_bag
        .iter()
        .zip(skill_in_bag_positions)
        .for_each(|(c, p)| {
            sprites.push(
                spawn_spell_in_bag(
                    &mut commands,
                    &texture_storage,
                    &p,
                    &template_storage,
                    c.id,
                    false
                )
            )
        });

    sprites.push(
        spawn_background_element(
            &mut commands,
            &texture_storage,
            Some(Vec2::new(10., 11.5)),
            Transform::from_xyz(7., -9., 600.),
            "Skill pack interface background",
        )
    );

    sprites.push(
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(11., -4.5, 700.),
            Exit,
            Element::Exit,
        )
    );

    update_event.send(UpdateEvent(true));

    let _ = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Skill pack interface"))
        .insert(BagInterface)
        .push_children(&sprites)
        .id();
}


pub fn spawn_bag_interface(
    mut commands: Commands,
    texture_storage: Res<TextureStorage>,
    template_storage: Res<TemplateStorage>,
    player_query: Query<&Player>,
) {
    let mut sprites = Vec::with_capacity(5);
    let player = player_query.single();
    let items = player.items_bag.clone();
    let build = &player.item_build;

    sprites.push(
        spawn_background_element(
            &mut commands,
            &texture_storage,
            Some(Vec2::new(10., 11.5)),
            Transform::from_xyz(7., -9., 600.),
            "Bag interface background",
        )
    );

    sprites.push(
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(11., -4.5, 700.),
            Exit,
            Element::Exit,
        )
    );


    let bag_item_positions = vec![
        Transform::from_xyz(3.0, -10., 700.),
        Transform::from_xyz(4.5, -10., 700.),
        Transform::from_xyz(6., -10., 700.),
        Transform::from_xyz(7.5, -10., 700.),
        Transform::from_xyz(9., -10., 700.),
        Transform::from_xyz(10.5, -10., 700.),
        Transform::from_xyz(3.0, -11.5, 700.),
        Transform::from_xyz(4.5, -11.5, 700.),
        Transform::from_xyz(6., -11.5, 700.),
        Transform::from_xyz(7.5, -11.5, 700.),
    ];

    items
        .iter()
        .zip(bag_item_positions.iter())
        .for_each(|(i, p)| {
            sprites.push(
                spawn_bag_item(
                    &mut commands,
                    &texture_storage,
                    *p,
                    &template_storage,
                    i.id,
                    true,
                )
            )
        });

    if let Some(def) = build.defense {
        sprites.push(
            spawn_bag_item(
                &mut commands,
                &texture_storage,
                Transform::from_xyz(5., -6.5, 700.),
                &template_storage,
                def.0,
                true,
            )
        );
    }

    if let Some(att) = build.attack {
        sprites.push(
            spawn_bag_item(
                &mut commands,
                &texture_storage,
                Transform::from_xyz(6., -6.5, 700.),
                &template_storage,
                att.0,
                true,
            )
        );
    }

    if let Some(mana) = build.mana {
        sprites.push(
            spawn_bag_item(
                &mut commands,
                &texture_storage,
                Transform::from_xyz(4., -6.5, 700.),
                &template_storage,
                mana.0,
                true,
            )
        );
    }

    if let Some(health) = build.health {
        sprites.push(
            spawn_bag_item(
                &mut commands,
                &texture_storage,
                Transform::from_xyz(5., -5.5, 700.),
                &template_storage,
                health.0,
                true,
            )
        );
    }


    let _ = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Bag interface"))
        .insert(BagInterface)
        .push_children(&sprites)
        .id();
}

pub fn spawn_world_interface(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    storage: Res<TemplateStorage>,
    frame_sheet: Res<FramesSheet>,
    texture_storage: Res<TextureStorage>,
) {
    let mut sprites = Vec::with_capacity(5);

    sprites.push(
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(14.2, -12., 200.),
            MoveDice {
                value: 0,
                can_roll: true,
            },
            Element::MoveDice,
        )
    );

    sprites.push(
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(14.2, -7., 200.),
            Backpack,
            Element::Backpack,
        )
    );

    sprites.push(
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(15.4, -7., 200.),
            SkillPack,
            Element::SkillPack,
        )
    );

    sprites.push(
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(15.4, -12., 200.),
            Next,
            Element::Next,
        )
    );

    sprites.push(
        spawn_background_element(
            &mut commands,
            &texture_storage,
            Some(Vec2::new(20., 25.)),
            Transform::from_xyz(7., -8., 1.),
            "World interface background",
        )
    );

    let _ = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("World interface"))
        .insert(WorldInterface)
        .push_children(&sprites)
        .id();
}

pub fn move_dice(
    selected_query: Query<&Selected, With<MoveDice>>,
    mut move_dice_query: Query<&mut MoveDice>,
) {
    let selected = selected_query.single();
    let mut move_dice = move_dice_query.single_mut();

    if selected.selected && move_dice.can_roll {
        let roll = thread_rng().gen_range(1..7);
        println!("{}", roll);
        move_dice.value = roll;
    }
}

pub fn player_encounter_checking(
    player_query: Query<&Transform, With<Player>>,
    encounter_query: Query<(&Transform, &EncounterType), (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
    mut encounter_event: EventWriter<EncounterEvent>,
) {
    let player_translation = player_query.single().translation;


    for (transform, enc_type) in encounter_query.iter()  {
        if collide_check(transform.translation, player_translation) {
            println!("Changing to Combat");
            encounter_event.send(EncounterEvent(enc_type.0));
            state.set(Combat).expect("Failed to change states");
        }
    }

    // if encounter_query
    //     .iter()
    //     .any(|&transform| collide_check(
    //         player_translation, transform.translation))
    // {
    //     println!("Changing to Combat");
    //     encounter_event.send()
    //     state.set(Combat).expect("Failed to change states");
    // }
}

pub fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

pub fn hide_buttons(
    children_query: Query<&Children, With<WorldInterface>>,
    mut buttons_query: Query<&mut Visibility, Without<WorldInterface>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = buttons_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

pub fn show_buttons(
    children_query: Query<&Children, With<WorldInterface>>,
    mut buttons_query: Query<&mut Visibility, Without<WorldInterface>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = buttons_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

pub fn show_player(
    mut player_query: Query<(&mut Player, &mut Visibility)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let (mut player, mut player_vis) = player_query.single_mut();
    // player.active = true;
    player_vis.is_visible = true;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

pub fn player_movement(
    mut player: Query<(&Player, &mut Transform), With<Player>>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    mut cursor_state: ResMut<CursorState>,
    buttons: Res<Input<MouseButton>>,
    mut move_points_query: Query<&mut MoveDice>,
) {
    let (pl, mut transform) = player.single_mut();
    let mut move_points = move_points_query.single_mut();

    if buttons.just_pressed(MouseButton::Right)  {
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

pub fn collide_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let collision = collide(
        target_player_pos,
        Vec2::splat(1.0),
        wall_translation,
        Vec2::splat(1.0),
    );
    collision.is_some()
}


pub fn spawn_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    storage: Res<TemplateStorage>,
    frame_sheet: Res<FramesSheet>,
    texture_storage: Res<TextureStorage>,
) {
    let item_build = ItemBuild {
        defense: None,
        attack: None,
        mana: None,
        health: None,
    };


    let mut items = Vec::new();

    for _ in 0..5 {
        items.push(storage.roll_item(1));
    }
    for _ in 0..5 {
        items.push(storage.roll_item(2));
    }
    for _ in 0..5 {
        items.push(storage.roll_item(3));
    }


    let mut combat_cards: Vec<CardView> = Vec::new();
    let mut cards_in_bag = Vec::new();

    // for _ in 0..3 {
    //     combat_cards.push(storage.roll_card(1));
    // }
    // for _ in 0..2 {
    //     combat_cards.push(storage.roll_card(2));
    // }
    // for _ in 0..1 {
    //     combat_cards.push(storage.roll_card(3));
    // }

    (1..=16)
        .for_each(|id| {
            cards_in_bag.push(storage.get_card(id))
        });


    if !combat_cards.is_empty() {
        combat_cards.sort_by(|a, b| a.level.cmp(&b.level));
    }


    let mut sprite = TextureAtlasSprite {
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
        .insert(Player {
            is_selected: false,
            combat_deck: combat_cards,
            items_bag: items,
            item_build,
            deck: cards_in_bag,
        })
        .insert(CombatStats {
            health: 10,
            defense: 1,
            attack: 20,
            max_health: 10,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true)
        })
        .insert(animation)
        .id();
}