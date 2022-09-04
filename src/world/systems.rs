use crate::player::*;
use crate::prelude::*;
use crate::world::components::*;


pub fn accept_reward(
    mut commands: Commands,
    selected_query: Query<&Selected, With<AcceptRewardButton>>,
    reward_interface_query: Query<Entity, With<WorldReward>>,
) {
    for selected in selected_query.iter() {
        if selected.selected {
            for entity in reward_interface_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn town_event(
    mut player_stats_query: Query<(&Transform, &mut CombatStats), With<Player>>,
    event_query: Query<&Transform, With<Town>>,
) {
    let (player_transform, mut player_stats) = player_stats_query.single_mut();
    let player_translation= player_transform.translation;

    for transform in event_query.iter() {

        if collide_check(transform.translation, player_translation) &&
            player_stats.health != player_stats.max_health
        {
            player_stats.health = player_stats.max_health;
        }
    }
}

pub fn world_object_event(
    mut commands: Commands,
    mut items: ResMut<ItemPull>,
    mut player_query: Query<&mut Player>,
    texture_storage: Res<TextureStorage>,
    template_storage: Res<TemplateStorage>,
    mut player_stats_query: Query<(&Transform, &mut CombatStats), With<Player>>,
    mut event_query: Query<(&Transform, &mut WorldEvent), (With<WorldEventMarker>, Without<Player>)>,
) {
    let (player_transform, mut player_stats) = player_stats_query.single_mut();
    let player_translation= player_transform.translation;
    let mut player = player_query.single_mut();

    for (transform, mut event) in event_query.iter_mut() {
        if collide_check(transform.translation, player_translation) && !event.is_visited {
            event.is_visited = true;

            if let Some(reward) = get_reward_template(&template_storage, event.lvl, &mut items) {
                match event.event_type {
                    WorldEventType::Camp => add_reward(&reward, &mut player),
                    WorldEventType::Ruins => add_reward(&reward, &mut player),
                    WorldEventType::Altar => {
                        add_reward(&reward, &mut player);
                        player_stats.max_health = std::cmp::max(player_stats.max_health - 1, 5_isize);
                    }
                }

                let window_translation = Vec3::new(
                    player_translation.x,
                    player_translation.y,
                    player_translation.z + 10.,
                );

                let text_translation = Vec3::new(
                    player_translation.x,
                    player_translation.y + 1.,
                    player_translation.z + 20.,
                );

                let sprites = vec![
                    spawn_reward_button(
                        &mut commands,
                        &reward,
                        &texture_storage,
                        Transform {
                            translation: window_translation,
                            ..default()
                        },
                        AcceptRewardButton
                    ),
                    spawn_background_element(
                        &mut commands,
                        &texture_storage,
                        Some(Vec2::new(5., 3.)),
                        Transform {
                            translation: window_translation,
                            ..default()
                        },
                        "Reward window",
                    ),
                    spawn_text(
                        &mut commands,
                        &texture_storage,
                        Transform {
                            translation: text_translation,
                            scale: Vec3::new(0.01, 0.01, 0.),
                            ..default()
                        },
                        "Your find".to_string(),
                        "Accept reward".to_string(),
                        AcceptRewardButton,
                        PlayerMarker,
                    ),
                ];

                let _ = commands
                    .spawn()
                    .insert(Transform::default())
                    .insert(GlobalTransform::default())
                    .insert(Name::new("World reward"))
                    .insert(WorldReward)
                    .push_children(&sprites)
                    .id();
            }
        }
    }
}

pub fn hide_buttons(
    children_query: Query<&Children, With<WorldInterface>>,
    buttons_query: Query<&mut Visibility, Without<WorldInterface>>,
) {
    toggle_visible(children_query, buttons_query, false);
}

pub fn show_buttons(
    children_query: Query<&Children, With<WorldInterface>>,
    buttons_query: Query<&mut Visibility, Without<WorldInterface>>,
) {
    toggle_visible(children_query, buttons_query, true);
}

pub fn add_reward(reward: &Reward, player: &mut Player) {
    match reward.entity_type {
        EntityType::SkillCard => player.add_in_deck(reward.item_id, reward.item_lvl),
        EntityType::Item => player.add_in_bag(reward.item_id, reward.item_lvl),
        _ => {}
    }
}

pub fn get_reward_template(
    template_storage: &Res<TemplateStorage>,
    mut lvl: usize,
    items: &mut ItemPull
) -> Option<Reward> {

    if lvl == 0 {
        lvl = thread_rng().gen_range(1..=3);
    }

    let template = if let Some(id) = items.get_item(lvl) {
        template_storage.types
            .iter()
            .find(|t| t.0 == id)
    } else {
        None
    };

    template.map(|template_| Reward {
        item_id: template_.0,
        entity_type: template_.1.clone(),
        sprite_index: template_.2,
        item_lvl: lvl
    })
}

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
        state.set(Deck).expect("Failed to change states");
    }
}

pub fn bag_button_exit(
    selected_query: Query<&Selected, With<BagExit>>,
    mut state: ResMut<State<GameState>>,
) {
    let selected = selected_query.single();
    if selected.selected {
        println!("exit selected");
        state.set(World).expect("Failed to change states");
    }
}

pub fn skill_pack_button_exit(
    selected_query: Query<&Selected, With<SkillPackExit>>,
    mut state: ResMut<State<GameState>>,
) {
    let selected = selected_query.single();
    if selected.selected {
        println!("exit selected");
        state.set(World).expect("Failed to change states");
    }
}

pub fn take_spell(
    mut selected_query: Query<(&mut CardView, &Selected), Without<CombatDeckSpell>>,
    mut player_query: Query<&mut Player>,
    mut update_event: EventWriter<UpdateEvent>,
) {
    let mut player = player_query.single_mut();
    for (item, selected) in selected_query.iter_mut() {
        if selected.selected && player.can_add(item.level) {
            player.add_spell(*item);
            update_event.send(UpdateEvent(true));
        }
    }
}

pub fn remove_spell(
    mut selected_query: Query<(&mut CardView, &Selected), With<CombatDeckSpell>>,
    mut player_query: Query<&mut Player>,
    mut update_event: EventWriter<UpdateEvent>,
) {
    let mut player = player_query.single_mut();
    for (item, selected) in selected_query.iter_mut() {
        if selected.selected {
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
    combat_deck_spells_query: Query<Entity, With<CombatDeckSpell>>,
) {
    let mut sprites = Vec::with_capacity(5);
    let mut player = player_query.single_mut();
    let combat_cards = &mut player.combat_deck;
    let interface = bag_interface_query.single_mut();

    if let Some(update_event) = update_event.iter().next() {
        if update_event.0 {
            for entity in combat_deck_spells_query.iter() {
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
                            true,
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
    for (item, selected, entity) in selected_query.iter_mut() {
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
    for entity in bag_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_stats(
    mut player_stats_query: Query<&mut CombatStats, With<Player>>,
    player_query: Query<&Player>
) {
    let mut stats = player_stats_query.single_mut();
    let player = player_query.single();
    if let Some(defense) = player.item_build.defense {
        stats.defense = defense.1 as isize;
    }
    if let Some(attack) = player.item_build.attack {
        stats.attack = attack.1 as isize;
    }
    if let Some(mana) = player.item_build.mana {
        stats.mana = mana.1 as isize;
    }
    if let Some(health) = player.item_build.health {
        stats.max_health += health.1 as isize;
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
    player_query: Query<&Player>,
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
                    false,
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
            SkillPackExit,
            Element::Exit,
        )
    );

    sprites.push(
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(15., -8.5, 700.),
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            "Update your deck before every battle.\n
            You can take into battle:\n
            - 3 first level spells\n
            - 2 second level spells\n
            - 1 third level spell.\n
            Mouse right click - cards info".to_string(),
            "Info text".to_string(),
            DeckInfoHint,
            PlayerMarker,
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
            BagExit,
            Element::Exit,
        )
    );

    sprites.push(
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(7., -14., 700.),
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            "Upgrade your items only when you receive new ones".to_string(),
            "Info text".to_string(),
            BagInfoHint,
            PlayerMarker,
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
                false,
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
                false,
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
                false,
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
                false,
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

pub fn update_move_points_text(
    text_query: Query<&mut Text, (With<MoveDiceText>, Without<EnemyMarker>)>,
    move_dice_query: Query<&MoveDice>
) {
    let move_points = move_dice_query.single().value;
    update_text(text_query,move_points);
}

pub fn update_days_count_text(
    text_query: Query<&mut Text, (With<DaysCountText>, Without<EnemyMarker>)>,
    next_button_query: Query<&NextButton>
) {
    let next_button = next_button_query.single().days;
    update_text(text_query,next_button as isize);
}

pub fn spawn_world_interface(
    mut commands: Commands,
    texture_storage: Res<TextureStorage>
) {
    let sprites = vec![
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(14.2, -12., 200.),
            MoveDice {
                value: 0,
                can_roll: true,
            },
            Element::MoveDice,
        ),
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(14.2, -7., 200.),
            Backpack,
            Element::Backpack,
        ),
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(15.4, -7., 200.),
            SkillPack,
            Element::SkillPack,
        ),
        spawn_world_interface_element(
            &mut commands,
            &texture_storage,
            Transform::from_xyz(15.4, -12., 200.),
            NextButton {days: 0},
            Element::Next,
        ),
        spawn_background_element(
            &mut commands,
            &texture_storage,
            Some(Vec2::new(20., 25.)),
            Transform::from_xyz(7., -8., 1.),
            "World interface background",
        ),
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(14.2, -11., 205.),
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            "0".to_string(),
            "Move dice text".to_string(),
            MoveDiceText,
            WorldTextMarker
        ),
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(15.4, -11., 205.),
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            "0".to_string(),
            "Days count".to_string(),
            DaysCountText,
            WorldTextMarker
        ),
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(15., -13., 205.),
                scale: Vec3::new(0.007, 0.007, 0.),
                ..default()
            },
            "Roll the movement cube\nto get points".to_string(),
            "Move info".to_string(),
            MoveInfoText,
            WorldTextMarker
        ),
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(15., -14., 205.),
                scale: Vec3::new(0.007, 0.007, 0.),
                ..default()
            },
            "When the turn is complete,\n move on to the next day".to_string(),
            "Next info".to_string(),
            NextInfoText,
            WorldTextMarker
        ),
    ];

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
        move_dice.can_roll = false;
        let roll = thread_rng().gen_range(1..7);
        move_dice.value = roll;
    }
}
pub fn next_day_button(
    selected_query: Query<&Selected, With<NextButton>>,
    mut move_dice_query: Query<&mut MoveDice>,
    mut next_button_query: Query<&mut NextButton>,
) {
    let selected = selected_query.single();
    let mut move_dice = move_dice_query.single_mut();
    let mut next_button = next_button_query.single_mut();
    if selected.selected && !move_dice.can_roll {
        next_button.increase();
        move_dice.can_roll = true;
        move_dice.value = 0_isize;
    }
}

pub fn toggle_visible<T: Component>(
    children_query: Query<&Children, With<T>>,
    mut buttons_query: Query<&mut Visibility, Without<T>>,
    show: bool
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = buttons_query.get_mut(*child) {
                child_vis.is_visible = show;
            }
        }
    }
}
