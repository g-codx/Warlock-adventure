use std::default::Default;
use std::fmt;
use std::fmt::Formatter;
use bevy::ecs::system::EntityCommands;
use crate::prelude::*;
use crate::combat::{EnemyType, Selected};


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PreStartup, Self::load_graphics,
            )
            .add_system(Self::frame_animation);
    }
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let lizard_image = assets.load("units/lizard.png");
        let lizard_atlas = TextureAtlas::from_grid(lizard_image, Vec2::new(256.0, 256.0), 4, 4);
        let lizard_atlas_handle = texture_atlases.add(lizard_atlas);

        let medusa_image = assets.load("units/duza.png");
        let medusa_atlas = TextureAtlas::from_grid(medusa_image, Vec2::new(128.0, 128.0), 4, 5);
        let medusa_atlas_handle = texture_atlases.add(medusa_atlas);

        let small_dragon_image = assets.load("units/small_dragon.png");
        let small_dragon_atlas = TextureAtlas::from_grid(small_dragon_image, Vec2::new(128.0, 128.0), 2, 6);
        let small_dragon_atlas_handle = texture_atlases.add(small_dragon_atlas);

        let gin_image = assets.load("units/djin.png");
        let gin_atlas = TextureAtlas::from_grid(gin_image, Vec2::new(128.0, 128.0), 8, 2);
        let gin_atlas_handle = texture_atlases.add(gin_atlas);

        let big_dragon_image = assets.load("units/big_dragon.png");
        let big_dragon_atlas = TextureAtlas::from_grid(big_dragon_image, Vec2::new(256.0, 256.0), 4, 4);
        let big_dragon_atlas_handle = texture_atlases.add(big_dragon_atlas);

        let demon_image = assets.load("units/demon.png");
        let demon_atlas = TextureAtlas::from_grid(demon_image, Vec2::new(256.0, 256.0), 4, 4);
        let demon_atlas_handle = texture_atlases.add(demon_atlas);


        let warlock_skills_image = assets.load("skills/warlock_skills.png");
        let warlock_skills_atlas = TextureAtlas::from_grid(warlock_skills_image, Vec2::new(256.0, 256.0), 4, 4);
        let warlock_card_handle = texture_atlases.add(warlock_skills_atlas);

        let glade = assets.load("interface/battleground1.png");
        let castle = assets.load("interface/battleground2.png");
        let forest = assets.load("interface/battleground3.png");
        let wasteland = assets.load("interface/battleground4.png");

        let combat_bar = assets.load("interface/bar_ready.png");
        let combat_border_frame = assets.load("interface/border_frame.png");

        let dice_image = assets.load("interface/dice.png");
        let dice_atlas = TextureAtlas::from_grid(dice_image, Vec2::new(256.0, 256.0), 4, 4);
        let dice_atlas_handle = texture_atlases.add(dice_atlas);

        let combat_icons = assets.load("interface/combat_icons.png");
        let combat_atlas = TextureAtlas::from_grid(combat_icons, Vec2::new(256.0, 256.0), 2, 3);
        let combat_icon_atlas_handle = texture_atlases.add(combat_atlas);

        let combat_hint_icons = assets.load("skills/hints.png");
        let combat_hint_atlas = TextureAtlas::from_grid(combat_hint_icons, Vec2::new(256.0, 256.0), 4, 4);
        let combat_hint_atlas_handle = texture_atlases.add(combat_hint_atlas);

        let font = assets.load("fonts/FiraSans-Bold.ttf");

        let base_image = assets.load("map/base_tiles_atlas.png");
        let base_atlas = TextureAtlas::from_grid(base_image, Vec2::new(256.0, 256.0), 2, 3);
        let base_tiles = texture_atlases.add(base_atlas);

        let object_tiles = assets.load("map/event_objects_atlas.png");
        let object_atlas = TextureAtlas::from_grid(object_tiles, Vec2::new(256.0, 256.0), 4, 4);
        let event_object_tiles = texture_atlases.add(object_atlas);

        let hero_idle_image = assets.load("player/hero_idle.png");
        let hero_idle_atlas = TextureAtlas::from_grid(hero_idle_image, Vec2::new(198.0, 198.0), 5, 4);
        let hero_idle_atlas_handle = texture_atlases.add(hero_idle_atlas);

        let world_interface_image = assets.load("interface/world_interface.png");
        let world_interface_atlas = TextureAtlas::from_grid(world_interface_image, Vec2::new(256.0, 256.0), 2, 3);
        let world_interface_handle = texture_atlases.add(world_interface_atlas);

        let items_image = assets.load("player/items.png");
        let items_atlas = TextureAtlas::from_grid(items_image, Vec2::new(256.0, 256.0), 4, 3);
        let items_atlas_handle = texture_atlases.add(items_atlas);


        commands.insert_resource(FramesSheet {
            lizard_handle: lizard_atlas_handle,
            lizard_idle: [13, 14, 15],
            lizard_attack: [0, 1, 2, 3, 4],
            lizard_death: [5, 6, 7, 8, 9, 10],
            lizard_hurt: [11, 12],

            medusa_handle: medusa_atlas_handle,
            medusa_attack: [0, 1, 2, 3, 4, 5],
            medusa_death: [6, 7, 8, 9, 10, 11],
            medusa_hurt: [12, 13],
            medusa_idle: [14, 15, 16],

            small_dragon_handle: small_dragon_atlas_handle,
            small_dragon_attack: [0, 1, 2],
            small_dragon_death: [3, 4, 5, 6],
            small_dragon_hurt: [7, 8],
            small_dragon_idle: [9, 10, 11],

            gin_handle: gin_atlas_handle,
            gin_attack: [0, 1, 2, 3],
            gin_death: [4, 5, 6, 7, 8, 9],
            gin_hurt: [10, 11],
            gin_idle: [12, 13, 14],

            big_dragon_handle: big_dragon_atlas_handle,
            big_dragon_attack: [0, 1, 2, 3],
            big_dragon_death: [4, 5, 6, 7, 8],
            big_dragon_hurt: [9, 10],
            big_dragon_idle: [11, 12, 13],

            demon_handle: demon_atlas_handle,
            demon_attack: [0, 1, 2, 3],
            demon_death: [4, 5, 6, 7, 8, 9],
            demon_hurt: [10, 11],
            demon_idle: [12, 13, 14],

            hero_idle_atlas_handle,
            hero_idle: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],

        });

        commands.insert_resource(TextureStorage {
            warlock_card_handle,
            glade,
            castle,
            forest,
            wasteland,
            combat_bar,
            combat_border_frame,
            dice_atlas_handle,
            combat_icon_atlas_handle,
            combat_hint_atlas_handle,
            font,
            base_tiles,
            event_object_tiles,
            world_interface_handle,
            items_atlas_handle,
        })
    }

    fn frame_animation(
        mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}

#[derive(Clone, Copy, Inspectable)]
pub enum AnimationType {
    Idle,
    Attack,
    Hurt,
    Death,
}


pub fn spawn_enemy_sprite(
    commands: &mut Commands,
    frame_sheet: &FramesSheet,
    enemy_type: EnemyType,
) -> Entity {
    let mut sprite = match enemy_type {
        EnemyType::Lizard => TextureAtlasSprite::new(frame_sheet.lizard_idle[0]),
        EnemyType::Medusa => TextureAtlasSprite::new(frame_sheet.medusa_idle[0]),
        EnemyType::SmallDragon => TextureAtlasSprite::new(frame_sheet.small_dragon_idle[0]),
        EnemyType::Gin => TextureAtlasSprite::new(frame_sheet.gin_idle[0]),
        EnemyType::BigDragon => TextureAtlasSprite::new(frame_sheet.big_dragon_idle[0]),
        EnemyType::Demon => TextureAtlasSprite::new(frame_sheet.demon_death[0]),
    };

    sprite.color = Color::rgb(1., 1., 1.);

    sprite.custom_size = match enemy_type {
        EnemyType::Medusa => Some(Vec2::new(3., 4.)),
        EnemyType::Gin => Some(Vec2::new(2.5, 3.2)),
        EnemyType::SmallDragon => Some(Vec2::new(3., 4.)),
        _ => Some(Vec2::new(3.5, 5.)),
    };

    let translation = match enemy_type {
        EnemyType::Lizard => Vec3::new(-1., 4., 300.0),
        EnemyType::Medusa => Vec3::new(-1., 4.3, 300.0),
        EnemyType::SmallDragon => Vec3::new(-1., 4., 300.0),
        EnemyType::Gin => Vec3::new(-1., 4.3, 300.0),
        EnemyType::BigDragon => Vec3::new(-1.2, 4.8, 300.0),
        EnemyType::Demon => Vec3::new(-1., 4.5, 300.0),
    };

    let animation = match enemy_type {
        EnemyType::Lizard => FrameAnimation {
            timer: Timer::from_seconds(0.7, true),
            frames: frame_sheet.lizard_idle.to_vec(),
            current_frame: 0,
        },
        EnemyType::Medusa => FrameAnimation {
            timer: Timer::from_seconds(0.7, true),
            frames: frame_sheet.medusa_idle.to_vec(),
            current_frame: 0,
        },
        EnemyType::SmallDragon => FrameAnimation {
            timer: Timer::from_seconds(0.7, true),
            frames: frame_sheet.small_dragon_idle.to_vec(),
            current_frame: 0,
        },
        EnemyType::Gin => FrameAnimation {
            timer: Timer::from_seconds(0.7, true),
            frames: frame_sheet.gin_idle.to_vec(),
            current_frame: 0,
        },
        EnemyType::BigDragon => FrameAnimation {
            timer: Timer::from_seconds(0.7, true),
            frames: frame_sheet.big_dragon_idle.to_vec(),
            current_frame: 0,
        },
        EnemyType::Demon => FrameAnimation {
            timer: Timer::from_seconds(0.7, true),
            frames: frame_sheet.demon_idle.to_vec(),
            current_frame: 0,
        },
    };

    let texture_atlas = match enemy_type {
        EnemyType::Lizard => frame_sheet.lizard_handle.clone(),
        EnemyType::Medusa => frame_sheet.medusa_handle.clone(),
        EnemyType::SmallDragon => frame_sheet.small_dragon_handle.clone(),
        EnemyType::Gin => frame_sheet.gin_handle.clone(),
        EnemyType::BigDragon => frame_sheet.big_dragon_handle.clone(),
        EnemyType::Demon => frame_sheet.demon_handle.clone(),
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas,
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        })
        .insert(animation)
        .id()
}

pub fn spawn_dice(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
    is_attack: bool,
    transform: Transform,
    component: impl Component,
    name: &'static str,
) -> Entity {
    let sprite = if is_attack {
        TextureAtlasSprite {
            custom_size: Some(Vec2::new(1., 1.)),
            index: 0,
            ..default()
        }
    } else {
        TextureAtlasSprite {
            custom_size: Some(Vec2::new(1., 1.)),
            index: 7,
            ..default()
        }
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_storage.dice_atlas_handle.clone(),
            transform,
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Interactive::default())
        .insert(Selected::default())
        .insert(component)
        .insert(Name::new(name))
        .id()
}

#[derive(Debug)]
pub enum Element {
    MoveDice,
    Backpack,
    SkillPack,
    Next,
    Exit,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn spawn_world_interface_element(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
    transform: Transform,
    component: impl Component,
    element: Element,
) -> Entity {
    let custom_size = Some(Vec2::new(1., 1.));

    let sprite = match element {
        Element::MoveDice => TextureAtlasSprite {
            custom_size,
            index: 3,
            ..default()
        },
        Element::SkillPack => TextureAtlasSprite {
            custom_size,
            index: 0,
            ..default()
        },
        Element::Backpack => TextureAtlasSprite {
            custom_size,
            index: 2,
            ..default()
        },
        Element::Next => TextureAtlasSprite {
            custom_size,
            index: 4,
            ..default()
        },
        Element::Exit => TextureAtlasSprite {
            custom_size,
            index: 1,
            ..default()
        },
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_storage.world_interface_handle.clone(),
            transform,
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Interactive::default())
        .insert(Selected::default())
        .insert(component)
        .insert(Name::new(element.to_string()))
        .id()
}


pub fn spawn_combat_button(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
    is_attack: bool,
    transform: Transform,
    component: impl Component,
    name: &'static str,
) -> Entity {
    let sprite = if is_attack {
        TextureAtlasSprite {
            custom_size: Some(Vec2::new(1., 1.)),
            index: 0,
            ..default()
        }
    } else {
        TextureAtlasSprite {
            custom_size: Some(Vec2::new(1., 1.)),
            index: 1,
            ..default()
        }
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_storage.combat_icon_atlas_handle.clone(),
            transform,
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Interactive::default())
        .insert(Selected::default())
        .insert(component)
        .insert(Name::new(name))
        .id()
}

pub fn spawn_reward_button(
    commands: &mut Commands,
    reward: &Reward,
    texture_storage: &TextureStorage,
    transform: Transform,
    component: impl Component,
) -> Entity {
    let sprite = TextureAtlasSprite {
        custom_size: Some(Vec2::new(1., 1.)),
        index: reward.sprite_index.unwrap(),
        ..default()
    };

    let texture_atlas = match reward.entity_type {
        EntityType::SkillCard => texture_storage.warlock_card_handle.clone(),
        _ => texture_storage.items_atlas_handle.clone(),
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas,
            transform,
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Interactive::default())
        .insert(Selected::default())
        .insert(component)
        .insert(Name::new("Reward"))
        .id()
}

pub fn spawn_combat_battleground(
    commands: &mut Commands,
    texture_storage: &TextureStorage,
    enemy_type: &EnemyType
) -> Entity {

    let background = match enemy_type {
        EnemyType::Lizard | EnemyType::Medusa => texture_storage.forest.clone(),
        EnemyType::Gin => texture_storage.glade.clone(),
        EnemyType::BigDragon | EnemyType::SmallDragon => texture_storage.castle.clone(),
        EnemyType::Demon => texture_storage.wasteland.clone(),
    };

    let color = match enemy_type {
        EnemyType::Lizard | EnemyType::Medusa => Color::rgb(0.0, 1.0, 1.0),
        EnemyType::Gin => Color::rgb(0.7, 1.0, 1.0),
        EnemyType::BigDragon | EnemyType::SmallDragon => Color::rgb(0.9, 0.9, 0.9),
        EnemyType::Demon => Color::rgb(0.75, 0.75, 0.75),
    };

    let flip_x = enemy_type == &EnemyType::SmallDragon
        || enemy_type == &EnemyType::BigDragon
        || enemy_type == &EnemyType::Demon;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(21.5, 12.0)),
                flip_x,
                ..default()
            },
            texture: background,
            transform: Transform::from_xyz(0., 0., 50.),
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Name::new("Battleground"))
        .insert(Battleground)
        .id()
}

pub fn spawn_bag_item(
    commands: &mut Commands,
    texture_storage: &TextureStorage,
    transform: Transform,
    template_storage: &TemplateStorage,
    id: usize,
    with_interactive: bool,
) -> Entity {
    let template = template_storage.items
        .iter()
        .find(|t| t.id == id)
        .unwrap();

    let sprite = TextureAtlasSprite {
        index: template.sprite_index.unwrap(),
        custom_size: Some(Vec2::new(1., 1.)),
        ..default()
    };

    let item = Item {
        id,
        lvl: template.level,
        name: template.name.to_string(),
        sprite_index: template.sprite_index,
        value: template.value.unwrap(),
        buff_type: template.card_action.as_ref().unwrap().clone(),
    };

    if with_interactive {
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: texture_storage.items_atlas_handle.clone(),
                transform,
                ..default()
            })
            .insert(Name::new(template.name.clone()))
            .insert(item)
            .insert(Selected::default())
            .insert(Interactive::default())
            .id()
    } else {
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: texture_storage.items_atlas_handle.clone(),
                transform,
                ..default()
            })
            .insert(Name::new(template.name.clone()))
            .insert(item)
            .id()
    }
}

pub fn spawn_combat_card(
    commands: &mut Commands,
    texture_storage: &TextureStorage,
    transform: &Transform,
    template_storage: &TemplateStorage,
    id: usize,
) -> Entity {
    let template = template_storage.skill_cards
        .iter()
        .filter(|t| t.id == id)
        .nth(0);

    if let Some(t) = template {
        let spell_sprite = TextureAtlasSprite {
            index: t.sprite_index.unwrap(),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        };

        let hint_sprite = TextureAtlasSprite {
            index: t.sub_sprite_index.unwrap(),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        };

        let card_component = Card {
            id: t.id,
            mana_cost: t.mana_cost.unwrap(),
            name: t.name.clone(),
            value: t.value.unwrap(),
            card_action: t.card_action.as_ref().unwrap().clone(),
            sprite_index: t.sprite_index.unwrap(),
            rounds: t.rounds.unwrap(),
            ..default()
        };

        let mut children = Vec::new();

        let spell_icon = commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: spell_sprite,
                texture_atlas: texture_storage.warlock_card_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0.5, transform.translation.z),
                    scale: Vec3::new(1.5, 1.5, 100.),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Spell sprite"))
            .id();

        children.push(spell_icon);

        let spell_hint = commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: hint_sprite,
                texture_atlas: texture_storage.combat_hint_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(0., -0.85, transform.translation.z),
                    ..default()
                },
                visibility: Visibility {
                    is_visible: true
                },
                ..default()
            })
            .insert(Name::new("Spell hint"))
            .id();

        children.push(spell_hint);

        spawn_background(commands, texture_storage, transform)
            .insert(GlobalTransform::default())
            .insert(Interactive::default())
            .insert(Selected::default())
            .insert(card_component)
            .insert(Name::new(t.name.clone()))
            .push_children(&children)
            .id()
    } else {
        spawn_background(commands, texture_storage, transform)
            .insert(GlobalTransform::default())
            .insert(Interactive::default())
            .insert(Selected::default())
            .insert(Name::new("Spell card"))
            .id()
    }
}

pub fn spawn_spell_in_bag(
    commands: &mut Commands,
    texture_storage: &TextureStorage,
    transform: &Transform,
    template_storage: &TemplateStorage,
    id: usize,
    is_combat_deck: bool,
) -> Entity {
    let template = template_storage.skill_cards
        .iter()
        .filter(|t| t.id == id)
        .nth(0);

    let spell_sprite = TextureAtlasSprite {
        index: template.unwrap().sprite_index.unwrap(),
        custom_size: Some(Vec2::new(1., 1.)),
        ..default()
    };

    if is_combat_deck {
        spawn_spell(
            commands,
            &texture_storage,
            &transform,
            spell_sprite,
        )
            .insert(CombatDeckSpell)
            .insert(Name::new("Spell sprite"))
            .insert(Selected::default())
            .insert(Interactive::default())
            .insert(CardView {
                id: template.unwrap().id,
                level: template.unwrap().level,
            })
            .id()
    } else {
        spawn_spell(
            commands,
            &texture_storage,
            &transform,
            spell_sprite,
        )
            .insert(Name::new("Spell sprite"))
            .insert(Selected::default())
            .insert(Interactive::default())
            .insert(CardView {
                id: template.unwrap().id,
                level: template.unwrap().level,
            })
            .id()
    }
}

fn spawn_spell<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    texture_storage: &TextureStorage,
    transform: &Transform,
    spell_sprite: TextureAtlasSprite,
) -> EntityCommands<'w, 's, 'a> {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: spell_sprite,
            texture_atlas: texture_storage.warlock_card_handle.clone(),
            transform: *transform,
            ..default()
        })
}


fn spawn_background<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    texture_storage: &TextureStorage,
    transform: &Transform,
) -> EntityCommands<'w, 's, 'a> {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(2., 3.)),
                ..default()
            },
            texture: texture_storage.combat_bar.clone(),
            transform: *transform,
            ..default()
        })
}


pub fn spawn_background_element(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
    custom_size: Option<Vec2>,
    transform: Transform,
    name: &'static str,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size,
                color: COMBAT_INTERFACE_COLOR,
                ..default()
            },
            texture: texture_storage.combat_bar.clone(),
            transform,
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Name::new(name))
        .id()
}

pub fn spawn_enemy_border_frame(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(2.5, 2.0)),
                color: COMBAT_INTERFACE_COLOR,
                ..default()
            },
            texture: texture_storage.combat_bar.clone(),
            transform: Transform::from_xyz(-1., 2., 101.),
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Name::new("Enemy border frame"))
        .id()
}

pub fn spawn_combat_icon(
    mut commands: &mut Commands,
    texture_storage: &TextureStorage,
    transform: Transform,
    index: usize,
    name: &'static str,
) -> Entity {
    let sprite = TextureAtlasSprite {
        custom_size: Some(Vec2::new(0.5, 0.5)),
        index,
        ..default()
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_storage.combat_icon_atlas_handle.clone(),
            transform,
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Name::new(name))
        .id()
}

pub fn spawn_text(
    commands: &mut Commands,
    texture_storage: &TextureStorage,
    transform: Transform,
    text: String,
    name: String,
    attribute_component: impl Component,
    entity_component: impl Component,
) -> Entity {
    let font = texture_storage.font.clone();

    let text_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::GOLD,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(text, text_style, text_alignment),
            transform,
            ..default()
        })
        .insert(Name::new(name))
        .insert(attribute_component)
        .insert(entity_component)
        .id()
}

pub fn spawn_tile(
    char: char,
    texture_storage: &TextureStorage,
    commands: &mut Commands,
    transform: Transform,
    enumerate: usize,
) -> Entity {
    let sprite = match char {
        '!' => TextureAtlasSprite {
            index: 0,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'o' => TextureAtlasSprite {
            index: 1,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        '2' => TextureAtlasSprite {
            index: 2,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        '3' => TextureAtlasSprite {
            index: 3,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        '1' => TextureAtlasSprite {
            index: 4,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },

        'a' => TextureAtlasSprite {
            index: 0,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'D' => TextureAtlasSprite {
            index: 1,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'C' => TextureAtlasSprite {
            index: 2,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'c' => TextureAtlasSprite {
            index: 3,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        '(' => TextureAtlasSprite {
            index: 4,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'F' => TextureAtlasSprite {
            index: 5,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        's' => TextureAtlasSprite {
            index: 6,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'f' => TextureAtlasSprite {
            index: 7,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'm' => TextureAtlasSprite {
            index: 8,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'M' => TextureAtlasSprite {
            index: 9,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'p' => TextureAtlasSprite {
            index: 10,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'r' => TextureAtlasSprite {
            index: 11,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'S' => TextureAtlasSprite {
            index: 12,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'd' => TextureAtlasSprite {
            index: 13,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        'j' => TextureAtlasSprite {
            index: 14,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        't' => TextureAtlasSprite {
            index: 15,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        _ => TextureAtlasSprite {
            index: 0,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
    };

    let texture_atlas = match char {
        '!' | 'o' | '1' | '2' | '3' => texture_storage.base_tiles.clone(),
        _ => texture_storage.event_object_tiles.clone()
    };

    let tile = commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas,
        transform,
        ..default()
    })
        .insert(Tile(enumerate))
        .id();

    match char {
        '1' | '2' | '3' => {
            commands.entity(tile)
                .insert(Name::new("Collider"))
                .insert(TileCollider);
        }
        'm' | 'M' | 'd' | 'j' | 's' | 'S' | 'p' | 'D' => {
            commands.entity(tile)
                .insert(EncounterSpawner)
                .insert(EncounterType(
                    EnemyType::get_type(char), false)
                )
                .insert(Name::new("EncounterPoint"))
                .insert(Point(transform));
        }
        'c' => {
            commands.entity(tile)
                .insert(WorldEventMarker)
                .insert(WorldEvent {
                    event_type: WorldEventType::Camp,
                    lvl: 1,
                    is_visited: false,
                })
                .insert(Name::new("Small camp"))
                .insert(Point(transform));
        }
        'C' => {
            commands.entity(tile)
                .insert(WorldEventMarker)
                .insert(WorldEvent {
                    event_type: WorldEventType::Camp,
                    lvl: 2,
                    is_visited: false,
                })
                .insert(Name::new("Middle camp"))
                .insert(Point(transform));
        }
        '(' => {
            commands.entity(tile)
                .insert(WorldEventMarker)
                .insert(WorldEvent {
                    event_type: WorldEventType::Camp,
                    lvl: 3,
                    is_visited: false,
                })
                .insert(Name::new("Big camp"))
                .insert(Point(transform));
        }
        'r' => {
            commands.entity(tile)
                .insert(WorldEventMarker)
                .insert(WorldEvent {
                    event_type: WorldEventType::Ruins,
                    lvl: 0,
                    is_visited: false,
                })
                .insert(Name::new("Ruins"))
                .insert(Point(transform));
        }
        'a' => {
            commands.entity(tile)
                .insert(WorldEventMarker)
                .insert(WorldEvent {
                    event_type: WorldEventType::Altar,
                    lvl: 0,
                    is_visited: false,
                })
                .insert(Name::new("Altar"))
                .insert(Point(transform));
        }
        '!' | 't' => {
            commands.entity(tile)
                .insert(Town)
                .insert(Name::new("Town"))
                .insert(Point(transform));
        }
        _ => {
            commands.entity(tile)
                .insert(Name::new("Point"))
                .insert(Point(transform));
        }
    };

    tile
}


pub struct FramesSheet {
    pub lizard_handle: Handle<TextureAtlas>,
    pub lizard_idle: [usize; 3],
    pub lizard_attack: [usize; 5],
    pub lizard_hurt: [usize; 2],
    pub lizard_death: [usize; 6],

    pub medusa_handle: Handle<TextureAtlas>,
    pub medusa_idle: [usize; 3],
    pub medusa_attack: [usize; 6],
    pub medusa_hurt: [usize; 2],
    pub medusa_death: [usize; 6],

    pub small_dragon_handle: Handle<TextureAtlas>,
    pub small_dragon_attack: [usize; 3],
    pub small_dragon_death: [usize; 4],
    pub small_dragon_hurt: [usize; 2],
    pub small_dragon_idle: [usize; 3],

    pub gin_handle: Handle<TextureAtlas>,
    pub gin_attack: [usize; 4],
    pub gin_death: [usize; 6],
    pub gin_hurt: [usize; 2],
    pub gin_idle: [usize; 3],

    pub big_dragon_handle: Handle<TextureAtlas>,
    pub big_dragon_attack: [usize; 4],
    pub big_dragon_death: [usize; 5],
    pub big_dragon_hurt: [usize; 2],
    pub big_dragon_idle: [usize; 3],

    pub demon_handle: Handle<TextureAtlas>,
    pub demon_attack: [usize; 4],
    pub demon_death: [usize; 6],
    pub demon_hurt: [usize; 2],
    pub demon_idle: [usize; 3],

    pub hero_idle_atlas_handle: Handle<TextureAtlas>,
    pub hero_idle: [usize; 16],
}

pub struct TextureStorage {
    pub warlock_card_handle: Handle<TextureAtlas>,
    pub glade: Handle<Image>,
    pub castle: Handle<Image>,
    pub forest: Handle<Image>,
    pub wasteland: Handle<Image>,
    pub combat_bar: Handle<Image>,
    pub combat_border_frame: Handle<Image>,
    pub dice_atlas_handle: Handle<TextureAtlas>,
    pub combat_icon_atlas_handle: Handle<TextureAtlas>,
    pub combat_hint_atlas_handle: Handle<TextureAtlas>,
    pub font: Handle<Font>,
    pub base_tiles: Handle<TextureAtlas>,
    pub event_object_tiles: Handle<TextureAtlas>,
    pub world_interface_handle: Handle<TextureAtlas>,
    pub items_atlas_handle: Handle<TextureAtlas>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}


