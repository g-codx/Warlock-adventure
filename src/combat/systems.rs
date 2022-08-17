use crate::combat::components::{AttackButton, AttackDice, AttackText, CombatManager, CombatState, CombatStats, DefenseText, Enemy, EnemyMarker, FightEvent, HealthText, HeroSpellButton, ManaDice, ManaText, PlayerMarker, RoundText, Selected};
use crate::combat::components::CombatState::{EnemyAttack, EnemyTurn, Finalize, PlayerAttack, PlayerTurn};
use crate::combat::components::EnemyType::Lizard;
use crate::CombatState::{End, EnemyDeath, Reward};
use crate::GameState;
use crate::KeyCode::Space;
use crate::player::{Card, EncounterTracker, Player};
use crate::prelude::*;

pub fn init_manager(
    player_stats_query: Query<(&CombatStats), With<Player>>,
    mut manager: ResMut<CombatManager>,
) {
    let player_stats = player_stats_query.single();
    manager.permanent_damage_buff = player_stats.attack;
    manager.permanent_defense_buff = player_stats.defense;
    manager.defense = manager.permanent_defense_buff;
    manager.damage = player_stats.attack;
}


pub fn finalize(
    mut manager: ResMut<CombatManager>,
    mut combat_state: ResMut<State<CombatState>>,
) {
    println!("FINALIZE");
    manager.damage = manager.permanent_damage_buff;
    manager.mana_poll = manager.permanent_mana_buff;
    manager.defense = manager.permanent_defense_buff;
    manager.can_roll_attack = true;
    manager.can_roll_mana = true;
    manager.round += 1;
    manager.skip_round = false;
    manager.enemy_skip_round = false;

    combat_state.set(PlayerTurn).unwrap();
    manager.print();
}

pub fn manager_default(mut manager: ResMut<CombatManager>) {
    manager.round = 1;
    manager.damage = 0;
    manager.mana_poll = 0;
    manager.defense = 0;
    manager.permanent_damage_buff = 0;
    manager.permanent_defense_buff = 0;
    manager.permanent_mana_buff = 0;
    manager.can_roll_mana = true;
    manager.can_roll_attack = true;
    manager.skip_round = false;
    manager.enemy_skip_round = false;
    manager.player_death = false;
    manager.enemy_death = false;
}


pub fn use_card(
    mut selected_query: Query<(&mut Card, &Selected, &Children)>,
    mut manager: ResMut<CombatManager>,
    mut player_stats_query: Query<&mut CombatStats, With<Player>>,
    mut player_query: Query<&mut Player>,
    mut visibility: Query<&mut Visibility, Without<Card>>,
) {
    let mut player_stats = player_stats_query.single_mut();
    let mut player = player_query.single_mut();
    for (mut card, selected, children) in selected_query.iter_mut() {
        if selected.selected && manager.if_can_cast(card.mana_cost, card.is_used) {
            match card.card_action {
                CardAction::AttackBuff => {
                    if CombatManager::is_range_buff(&card) {
                        manager.permanent_damage_buff += card.value as isize;
                    } else {
                        manager.damage += card.value as isize;
                    }

                    manager.print();
                }
                CardAction::DefenceBuff => {
                    manager.permanent_defense_buff += card.value as isize;
                    manager.print();
                }
                CardAction::HealthBuff => {
                    if player_stats.health + card.value as isize > player_stats.max_health {
                        player_stats.health = player_stats.max_health;
                    } else {
                        player_stats.health += card.value as isize;
                    }
                    manager.print();
                }
                CardAction::ManaBuff => {
                    manager.permanent_mana_buff += card.value as isize;
                    manager.print();
                }
                CardAction::Special => {
                    match card.name.as_ref() {
                        "Fear" => {
                            manager.enemy_skip_round = true;
                        }
                        "Spirit Seance" => {
                            manager.permanent_damage_buff = manager.round as isize;
                            manager.damage += manager.round as isize;
                        }
                        "Takeover" => {
                            player_stats.max_health += 1;
                            player_stats.health += 1;
                        }
                        _ => {}
                    }
                }
            }

            let index = player.combat_deck
                .iter()
                .position(|c| c.id == card.id)
                .unwrap();
            player.combat_deck.remove(index);


            for child in children.iter() {
                if let Ok(mut vis) = visibility.get_mut(*child) {
                    vis.is_visible = false;
                }
            }

            card.is_used = true;
        }
    }
}


pub fn update_mana_poll_text(
    mut text_query: Query<(&mut Text), (With<ManaText>, Without<EnemyMarker>)>,
    manager: Res<CombatManager>,
) {
    update_text(text_query, manager.mana_poll);
}

pub fn update_damage_text(
    mut text_query: Query<(&mut Text), (With<AttackText>, Without<EnemyMarker>)>,
    manager: Res<CombatManager>,
) {
    update_text(text_query, manager.damage);
}

pub fn update_defense_text(
    mut text_query: Query<(&mut Text), (With<DefenseText>, Without<EnemyMarker>)>,
    manager: Res<CombatManager>,
) {
    update_text(text_query, manager.defense);
}

pub fn update_health_text(
    mut text_query: Query<(&mut Text), (With<HealthText>, Without<EnemyMarker>)>,
    player_query: Query<&CombatStats, With<Player>>,
) {
    let player_hp = player_query.single().health;
    update_text(text_query, player_hp);
}

pub fn update_round_text(
    mut text_query: Query<(&mut Text), (With<RoundText>, Without<EnemyMarker>)>,
    manager: Res<CombatManager>,
) {
    update_text(text_query, manager.round as isize);
}

pub fn update_enemy_health_text(
    mut text_query: Query<(&mut Text), (With<HealthText>, Without<PlayerMarker>)>,
    enemy_query: Query<&CombatStats, With<Enemy>>,
) {
    let enemy_hp = enemy_query.single().health;
    update_text(text_query, enemy_hp);
}

pub fn update_attack_dice_sprite(mut dice_query: Query<(&mut TextureAtlasSprite), With<AttackDice>>) {
    update_dice_sprite(dice_query, true);
}

pub fn update_mana_dice_sprite(mut dice_query: Query<(&mut TextureAtlasSprite), With<ManaDice>>) {
    update_dice_sprite(dice_query, false);
}


pub fn update_dice_sprite<T: Component>(
    mut dice_query: Query<(&mut TextureAtlasSprite), With<T>>,
    is_attack: bool,
) {
    let mut dice_sprite = dice_query.single_mut();
    dice_sprite.index = if is_attack { 0 } else { 7 };
}


pub fn update_text<T: Component, M: Component>(
    mut text_query: Query<(&mut Text), (With<T>, Without<M>)>,
    value: isize,
) {
    let mut text = text_query.single_mut();

    text.sections.iter_mut().for_each(|mut s| {
        s.value = value.to_string();
    });
}


pub fn attack_dice_roll(
    mut selected_query: Query<(&Selected, &mut TextureAtlasSprite), With<AttackDice>>,
    mut manager: ResMut<CombatManager>,
    mut text_query: Query<(&mut Text), (With<AttackText>, Without<EnemyMarker>)>,
) {
    let (selected, mut atlas) = selected_query.single_mut();

    if selected.selected && manager.can_roll_attack {
        let roll = thread_rng().gen_range(1..7);
        manager.damage += roll;
        manager.can_roll_attack = false;
        atlas.index = roll as usize;

        update_text(text_query, manager.damage);
        manager.print();
    }
}

pub fn mana_dice_roll(
    mut selected_query: Query<(&Selected, &mut TextureAtlasSprite), With<ManaDice>>,
    mut manager: ResMut<CombatManager>,
    mut text_query: Query<(&mut Text), (With<ManaText>, Without<EnemyMarker>)>,
) {
    let (selected, mut atlas) = selected_query.single_mut();

    if selected.selected && manager.can_roll_mana {
        let roll = thread_rng().gen_range(1..7);
        manager.mana_poll += roll;
        manager.can_roll_mana = false;
        atlas.index = 7 + roll as usize;

        update_text(text_query, manager.mana_poll);
        manager.print();
    }
}

pub fn attack_button(
    selected_query: Query<&Selected, With<AttackButton>>,
    mut manager: ResMut<CombatManager>,
    mut fight_event: EventWriter<FightEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let mut selected = selected_query.single().selected;
    let enemy = enemy_query.single();

    if selected && !manager.can_roll_attack && !manager.skip_round {
        println!("attack");
        fight_event.send(FightEvent {
            target: enemy,
            damage_amount: manager.damage,
            next_state: PlayerAttack,
        });
    }
}


pub fn skip_button(
    selected_query: Query<&Selected, With<HeroSpellButton>>,
    mut manager: ResMut<CombatManager>,
    mut combat_state: ResMut<State<CombatState>>,
    time: Res<Time>,
) {
    let mut selected = selected_query.single().selected;

    if selected && !manager.can_roll_attack && !manager.skip_round {
        manager.permanent_mana_buff += 1;
        manager.skip_round = true;
        manager.print();
    }

    manager.timer.tick(time.delta());

    if manager.timer.just_finished() && manager.skip_round && manager.enemy_skip_round {
        combat_state.set(Finalize).unwrap();
    } else if manager.timer.just_finished() && manager.skip_round && !manager.enemy_skip_round {
        combat_state.set(EnemyTurn).unwrap();
    }
}

pub fn damage_calculation(
    mut fight_event: EventReader<FightEvent>,
    mut target_query: Query<&mut CombatStats>,
    mut combat_state: ResMut<State<CombatState>>,
    mut manager: ResMut<CombatManager>,
) {
    if combat_state.current() == &Finalize
        || combat_state.current() == &PlayerAttack
        || combat_state.current() == &EnemyAttack
        || combat_state.current() == &End
        || combat_state.current() == &EnemyDeath
    {
        return;
    }

    if let Some(fight_event) = fight_event.iter().next() {
        let mut target_stats = target_query
            .get_mut(fight_event.target)
            .expect("Fight target without stats!");

        let pure_damage = if combat_state.current() == &PlayerTurn {
            std::cmp::max(
                fight_event.damage_amount - target_stats.defense,
                0,
            )
        } else {
            std::cmp::max(
                fight_event.damage_amount - manager.defense,
                0,
            )
        };

        target_stats.health = std::cmp::max(target_stats.health - pure_damage, 0);
        if target_stats.health == 0 {
            if fight_event.next_state == PlayerAttack {
                manager.enemy_death = true;
                combat_state.set(EnemyDeath).unwrap();
                return;
            } else {
                manager.player_death = true;
            }
        }

        combat_state.set(fight_event.next_state).unwrap();
    }
}


pub fn combat_end_button(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut manager: ResMut<CombatManager>,
    selected_query: Query<&Selected, With<CombatEndButton>>,
    mut state: ResMut<State<GameState>>,
) {
    println!("end");
    if state.current() == &World {
        return;
    }
    let mut selected = selected_query.single().selected;
    let mut transform = player_transform_query.single_mut();

    if selected && manager.enemy_death {
        println!("Changing to World");
        //todo логика получения награды и спавна в мире в той же точке
        transform.translation = Vec3::new(2.0, -2.0, 900.0);
        state.set(World).unwrap();
    }

    if selected && manager.player_death {
        println!("Changing to World");
        //todo логика потери шмота при смерти и спавна в городе
        transform.translation = Vec3::new(2.0, -2.0, 900.0);
        state.set(World).unwrap();
    }
}


pub fn end_combat(
    mut commands: Commands,
    mut manager: ResMut<CombatManager>,
    texture_storage: Res<TextureStorage>,
) {
    println!("END");

    let mut sprites = Vec::with_capacity(3);

    sprites.push(
        spawn_background_element(
            &mut commands,
            &texture_storage,
            Some(Vec2::new(5., 3.)),
            Transform::from_xyz(-1., 1., 700.),
            "End window",
        )
    );

    sprites.push(
        spawn_combat_button(
            &mut commands,
            &texture_storage,
            false,
            Transform::from_xyz(-1., 0.5, 800.),
            CombatEndButton,
            "End button",
        )
    );

    let text = if manager.enemy_death { "Get reward" } else { "Respawn" };

    sprites.push(
        spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: Vec3::new(-1., 1.5, 800.0),
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            text.to_string(),
            "Combat end text".to_string(),
            CombatEndButton,
            PlayerMarker,
        )
    );

    let _ = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Combat end interface"))
        .insert(TopItems)
        .push_children(&sprites)
        .id();
}

pub fn enemy_turn(
    mut fight_event: EventWriter<FightEvent>,
    enemy_query: Query<&CombatStats, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_ent = player_query.single();
    let enemy_stats = enemy_query.iter().next().unwrap();

    fight_event.send(FightEvent {
        target: player_ent,
        damage_amount: enemy_stats.attack,
        next_state: EnemyAttack,
    });
}


pub fn enemy_attack_effect(
    mut enemy_frame_query: Query<(&mut FrameAnimation, &mut EncounterTracker), With<Enemy>>,
    mut combat_state: ResMut<State<CombatState>>,
    frame_sheet: ResMut<FramesSheet>,
    time: Res<Time>,
    mut manager: ResMut<CombatManager>,
    enemy_query: Query<&Enemy>
) {

    let enemy_type = enemy_query.single().enemy_type;

    let idle_frames = match enemy_type {
        Lizard => frame_sheet.lizard_idle.to_vec(),
        EnemyType::Medusa => frame_sheet.medusa_idle.to_vec(),
        EnemyType::SmallDragon => frame_sheet.small_dragon_idle.to_vec(),
        EnemyType::Gin => frame_sheet.gin_idle.to_vec(),
        EnemyType::BigDragon => frame_sheet.big_dragon_idle.to_vec(),
        EnemyType::Demon => frame_sheet.demon_idle.to_vec(),
    };

    let attack_frames = match enemy_type {
        Lizard => frame_sheet.lizard_attack.to_vec(),
        EnemyType::Medusa => frame_sheet.medusa_attack.to_vec(),
        EnemyType::SmallDragon => frame_sheet.small_dragon_attack.to_vec(),
        EnemyType::Gin => frame_sheet.gin_attack.to_vec(),
        EnemyType::BigDragon => frame_sheet.big_dragon_attack.to_vec(),
        EnemyType::Demon => frame_sheet.demon_attack.to_vec(),
    };

    let hurt_frames = match enemy_type {
        Lizard => frame_sheet.lizard_hurt.to_vec(),
        EnemyType::Medusa => frame_sheet.medusa_hurt.to_vec(),
        EnemyType::SmallDragon => frame_sheet.small_dragon_hurt.to_vec(),
        EnemyType::Gin => frame_sheet.gin_hurt.to_vec(),
        EnemyType::BigDragon => frame_sheet.big_dragon_hurt.to_vec(),
        EnemyType::Demon => frame_sheet.demon_hurt.to_vec(),
    };

    let death_frames = match enemy_type {
        Lizard => frame_sheet.lizard_death.to_vec(),
        EnemyType::Medusa => frame_sheet.medusa_death.to_vec(),
        EnemyType::SmallDragon => frame_sheet.small_dragon_death.to_vec(),
        EnemyType::Gin => frame_sheet.gin_death.to_vec(),
        EnemyType::BigDragon => frame_sheet.big_dragon_death.to_vec(),
        EnemyType::Demon => frame_sheet.demon_death.to_vec(),
    };

    let last_death_frame = match enemy_type {
        Lizard => death_frames[5],
        EnemyType::Medusa => death_frames[5],
        EnemyType::SmallDragon => death_frames[3],
        EnemyType::Gin => death_frames[5],
        EnemyType::BigDragon => death_frames[4],
        EnemyType::Demon => death_frames[5],
    };


    for (mut effect, mut encounter_tracker) in enemy_frame_query.iter_mut() {
        encounter_tracker.timer.tick(time.delta());

        match combat_state.current() {
            PlayerTurn => effect.frames = idle_frames.clone(),
            PlayerAttack => {
                println!("PlayerAttack state");
                effect.frames = hurt_frames.clone();

                if encounter_tracker.timer.just_finished() && manager.enemy_death {
                    combat_state.set(End).unwrap();
                }

                if encounter_tracker.timer.just_finished() && manager.enemy_skip_round {
                    combat_state.set(Finalize).unwrap();
                } else if encounter_tracker.timer.just_finished() && !manager.enemy_skip_round {
                    combat_state.set(EnemyTurn).unwrap();
                }
            }
            EnemyAttack => {
                println!("EnemyAttack state");
                effect.frames = attack_frames.clone();

                if encounter_tracker.timer.just_finished() {
                    if manager.player_death {
                        combat_state.set(End).unwrap();
                    } else {
                        combat_state.set(Finalize).unwrap();
                    }
                }
            }
            EnemyDeath => {
                effect.frames = death_frames.clone();
                if encounter_tracker.timer.just_finished() {
                    combat_state.set(End).unwrap();
                }
            }
            End => {
                if manager.enemy_death {
                    effect.frames = vec![last_death_frame];
                } else {
                    effect.frames = idle_frames.clone();
                }
            }
            _ => {}
        }
    }
}

pub fn set_starting_state(mut state: ResMut<State<CombatState>>) {
    let _ = state.set(PlayerTurn);
}


pub fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

pub fn despawn_bottom_items(
    mut commands: Commands,
    enemy_query: Query<Entity, With<BottomItems>>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_top_items(
    mut commands: Commands,
    enemy_query: Query<Entity, With<TopItems>>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for (entity) in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_player_text(
    mut commands: Commands, enemy_query: Query<Entity, With<PlayerMarker>>,
) {
    for (entity) in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_battleground(
    mut commands: Commands, enemy_query: Query<Entity, With<Battleground>>,
) {
    for (entity) in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    frame_sheet: Res<FramesSheet>,
    texture_storage: Res<TextureStorage>,
    template_storage: Res<TemplateStorage>,
    mut encounter_event: EventReader<EncounterEvent>,
) {
    if let Some(event) = encounter_event.iter().next() {

        let enemy_type = event.0;
        let enemy_stats = template_storage.get_enemy(enemy_type).unwrap();

        let attack_text_translation = match enemy_type {
            Lizard => Vec3::new(-0.2, -1.4, 205.0),
            EnemyType::Medusa => Vec3::new(-0.2, -1.7, 205.0),
            EnemyType::SmallDragon => Vec3::new(-0.2, -1.4, 205.0),
            EnemyType::Gin => Vec3::new(-0.2, -1.7, 205.0),
            EnemyType::BigDragon => Vec3::new(0., -2.2, 205.0),
            EnemyType::Demon => Vec3::new(-0.2, -1.9, 205.0),
        };

        let defense_text_translation = match enemy_type {
            Lizard => Vec3::new(-0.2, -2., 205.0),
            EnemyType::Medusa => Vec3::new(-0.2, -2.3, 205.0),
            EnemyType::SmallDragon => Vec3::new(-0.2, -2., 205.0),
            EnemyType::Gin => Vec3::new(-0.2, -2.3, 205.0),
            EnemyType::BigDragon => Vec3::new(0., -2.8, 205.0),
            EnemyType::Demon => Vec3::new(-0.2, -2.5, 205.0),
        };

        let health_text_translation = match enemy_type {
            Lizard => Vec3::new(-0.2, -2.6, 205.0),
            EnemyType::Medusa => Vec3::new(-0.2, -2.9, 205.0),
            EnemyType::SmallDragon => Vec3::new(-0.2, -2.6, 205.0),
            EnemyType::Gin => Vec3::new(-0.2, -2.9, 205.0),
            EnemyType::BigDragon => Vec3::new(0., -3.4, 205.0),
            EnemyType::Demon => Vec3::new(-0.2, -3.1, 205.0),
        };

        let attack_text = spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: attack_text_translation,
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            enemy_stats.attack.unwrap().to_string(),
            "Enemy attack text".to_string(),
            AttackText,
            EnemyMarker,
        );

        let defense_text = spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: defense_text_translation,
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            enemy_stats.defense.unwrap().to_string(),
            "Enemy defense text".to_string(),
            DefenseText,
            EnemyMarker,
        );

        let health_text = spawn_text(
            &mut commands,
            &texture_storage,
            Transform {
                translation: health_text_translation,
                scale: Vec3::new(0.01, 0.01, 0.),
                ..default()
            },
            enemy_stats.health.unwrap().to_string(),
            "Enemy health text".to_string(),
            HealthText,
            EnemyMarker,
        );


        let sprite = spawn_enemy_sprite(
            &mut commands,
            &frame_sheet,
            enemy_type,
        );

        commands
            .entity(sprite)
            .insert(Enemy { enemy_type })
            .insert(CombatStats {
                health: enemy_stats.health.unwrap() as isize,
                attack: enemy_stats.attack.unwrap() as isize,
                defense: enemy_stats.defense.unwrap() as isize,
                max_health: enemy_stats.health.unwrap() as isize,
            })
            .insert(Name::new("Enemy"))
            .insert(EncounterTracker {
                timer: Timer::from_seconds(3.5, true)
            })
            .push_children(&[attack_text])
            .push_children(&[defense_text])
            .push_children(&[health_text]);
    };
}

pub fn spawn_interface(
    mut commands: Commands,
    texture_storage: Res<TextureStorage>,
    player_query: Query<&Player>,
    storage: Res<TemplateStorage>,
    player_stats_query: Query<(&CombatStats), With<Player>>,
) {
    let _bottom = spawn_bottom_bar(
        &mut commands,
        &texture_storage,
        player_query,
        &storage,
        player_stats_query,
    );

    let _top = spawn_top_bar(&mut commands, &texture_storage);
    let _battleground = spawn_combat_battleground(&mut commands, &texture_storage);
}

pub fn spawn_top_bar(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
) -> Entity {
    let mut sprites = Vec::new();

    sprites.push(spawn_background_element(
        commands,
        texture_storage,
        Some(Vec2::new(3., 5.)),
        Transform::from_xyz(-1., 3., 100.),
        "Enemy background",
    ));

    sprites.push(spawn_enemy_border_frame(commands, texture_storage));

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(-1.8, 2.6, 150.),
            2,
            "Enemy attack icon",
        )
    );

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(-1.8, 2., 150.),
            5,
            "Enemy defense icon",
        )
    );

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(-1.8, 1.4, 150.),
            4,
            "Enemy health icon",
        )
    );

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Top Items"))
        .insert(TopItems)
        .push_children(&sprites)
        .id()
}

pub fn spawn_player_text(
    mut commands: &mut Commands,
    player_stats_query: Query<(&CombatStats), With<Player>>,
    texture_storage: &Res<TextureStorage>,
) -> Entity {
    let player_stats = player_stats_query.single();

    let attack_text = spawn_text(
        commands,
        texture_storage,
        Transform {
            translation: Vec3::new(7.2, -3., 205.0),
            scale: Vec3::new(0.01, 0.01, 0.),
            ..default()
        },
        player_stats.attack.to_string(),
        "Player attack text".to_string(),
        AttackText,
        PlayerMarker,
    );
    let defense_text = spawn_text(
        commands,
        texture_storage,
        Transform {
            translation: Vec3::new(7.2, -3.7, 205.0),
            scale: Vec3::new(0.01, 0.01, 0.),
            ..default()
        },
        player_stats.defense.to_string(),
        "Player defense text".to_string(),
        DefenseText,
        PlayerMarker,
    );

    let mana_text = spawn_text(
        commands,
        texture_storage,
        Transform {
            translation: Vec3::new(7.2, -4.4, 205.0),
            scale: Vec3::new(0.01, 0.01, 0.),
            ..default()
        },
        0.to_string(),
        "Player mana text".to_string(),
        ManaText,
        PlayerMarker,
    );

    let health_text = spawn_text(
        commands,
        texture_storage,
        Transform {
            translation: Vec3::new(7.2, -5.1, 205.0),
            scale: Vec3::new(0.01, 0.01, 0.),
            ..default()
        },
        player_stats.health.to_string(),
        "Player health text".to_string(),
        HealthText,
        PlayerMarker,
    );

    let round_text = spawn_text(
        commands,
        texture_storage,
        Transform {
            translation: Vec3::new(7.9, -0.5, 205.0),
            scale: Vec3::new(0.01, 0.01, 0.),
            ..default()
        },
        "1".to_string(),
        "Round text".to_string(),
        RoundText,
        PlayerMarker,
    );

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Player stat text"))
        .insert(PlayerMarker)
        .push_children(&[attack_text])
        .push_children(&[defense_text])
        .push_children(&[mana_text])
        .push_children(&[health_text])
        .push_children(&[round_text])
        .id()
}

pub fn spawn_bottom_bar(
    mut commands: &mut Commands,
    texture_storage: &Res<TextureStorage>,
    player_query: Query<&Player>,
    template_storage: &TemplateStorage,
    player_stats_query: Query<(&CombatStats), With<Player>>,
) -> Entity {
    let player = player_query.single();
    let mut sprites = Vec::new();
    let cards_id = player.combat_deck.clone();

    let _text_stats = spawn_player_text(commands, player_stats_query, texture_storage);

    let positions = vec![
        Transform::from_xyz(-8.5, -4., 100.),
        Transform::from_xyz(-6., -4., 100.),
        Transform::from_xyz(-3.5, -4., 100.),
        Transform::from_xyz(-1., -4., 100.),
        Transform::from_xyz(1.5, -4., 100.),
        Transform::from_xyz(4., -4., 100.),
    ];

    let mut pos = positions.iter();
    let mut cards = cards_id.iter();

    loop {
        match (pos.next(), cards.next()) {
            (Some(transform), Some(card)) => {
                let card = spawn_combat_card(
                    commands,
                    texture_storage,
                    transform,
                    template_storage,
                    card.id,
                );

                sprites.push(card);
            }

            (Some(transform), None) => {
                let card = spawn_combat_card(
                    commands,
                    texture_storage,
                    transform,
                    template_storage,
                    0,
                );

                sprites.push(card);
            }
            _ => break
        }
    }

    sprites.push(
        spawn_dice(
            commands,
            texture_storage,
            true,
            Transform::from_xyz(9., -4.7, 200.),
            AttackDice,
            "Attack dice",
        )
    );

    sprites.push(
        spawn_dice(
            commands,
            texture_storage,
            false,
            Transform::from_xyz(9., -3.3, 200.),
            ManaDice,
            "Mana dice",
        )
    );

    sprites.push(
        spawn_background_element(
            commands,
            texture_storage,
            Some(Vec2::new(4., 3.)),
            Transform::from_xyz(8., -4., 100.),
            "Player stats background",
        )
    );

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(6.5, -3.02, 150.),
            2,
            "Attack icon",
        )
    );

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(6.5, -3.7, 150.),
            5,
            "Defense icon",
        )
    );

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(6.5, -4.4, 150.),
            3,
            "Mana icon",
        )
    );

    sprites.push(
        spawn_combat_icon(
            commands,
            texture_storage,
            Transform::from_xyz(6.5, -5.1, 150.),
            4,
            "Health icon",
        )
    );

    sprites.push(
        spawn_background_element(
            commands,
            texture_storage,
            Some(Vec2::new(4., 2.)),
            Transform::from_xyz(8., -1.0, 100.),
            "Player turn bar background",
        )
    );

    sprites.push(
        spawn_combat_button(
            commands,
            texture_storage,
            true,
            Transform::from_xyz(7., -1., 200.),
            AttackButton,
            "Attack button",
        )
    );

    sprites.push(
        spawn_combat_button(
            commands,
            texture_storage,
            false,
            Transform::from_xyz(9.0, -1., 200.),
            HeroSpellButton,
            "Hero spell button",
        )
    );

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(BottomItems)
        .insert(Name::new("Bottom Items"))
        .push_children(&sprites)
        .id()
}