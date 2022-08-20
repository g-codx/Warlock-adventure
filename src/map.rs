use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::prelude::*;
use crate::WorldEventType::{Altar, Camp};


pub struct MapPlugin;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
pub struct EncounterType(pub EnemyType, pub bool);


#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Point(pub Transform);

#[derive(Component, Inspectable, Default)]
pub struct Tile(pub usize);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_system(create_simple_map)
            .add_startup_system(create_map)
            .add_system_set(SystemSet::on_enter(World).with_system(show_map))
            .add_system_set(SystemSet::on_exit(World).with_system(hide_map));
    }
}

fn hide_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}


fn create_map(
    mut commands: Commands,
    texture_storage: Res<TextureStorage>,
) {
    let file = File::open("assets/map/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                tiles.push(
                    spawn_tile(
                        char,
                        &texture_storage,
                        &mut commands,
                        Transform::from_translation(Vec3::new(
                            x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.,
                        )),
                        x,
                    )
                );
            }
        }
    }
    commands
        .spawn()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}