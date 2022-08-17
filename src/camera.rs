use crate::KeyCode::{A, D, S, W};
use crate::MouseButton::Left;
use crate::player::Player;
use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system_set(
                SystemSet::on_update(World)
                    .with_system(camera_motion)
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.scale = CAMERA_SCALE;

    camera.transform = Transform::from_xyz(10.0, -10.0, 1000.0);

    commands.spawn_bundle(camera);
}

// fn camera_motion(
//     mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
//     windows: Res<Windows>,
//     time: Res<Time>
// ) {
//     let mut camera_transform = camera_query.single_mut();
//     let window = windows.get_primary().unwrap();
//
//     if let Some(screen_pos) = window.cursor_position() {
//         let y_cursor_delta = window.height() - screen_pos.y;
//         let x_cursor_delta = window.width() - screen_pos.x;
//
//         if CAMERA_MOVE_BORDER.contains(&y_cursor_delta) {
//             camera_transform.translation.y += CAMERA_SPEED * time.delta_seconds() ;
//         } else if CAMERA_MOVE_BORDER.contains(&screen_pos.y) {
//             camera_transform.translation.y -= CAMERA_SPEED * time.delta_seconds();
//         } else if CAMERA_MOVE_BORDER.contains(&x_cursor_delta) {
//             camera_transform.translation.x += CAMERA_SPEED * time.delta_seconds();
//         } else if CAMERA_MOVE_BORDER.contains(&screen_pos.x) {
//             camera_transform.translation.x -= CAMERA_SPEED * time.delta_seconds();
//         }
//
//     }
// }

fn camera_motion(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    windows: Res<Windows>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let window = windows.get_primary().unwrap();


    if let Some(screen_pos) = window.cursor_position() {
        let y_cursor_delta = window.height() - screen_pos.y;
        let x_cursor_delta = window.width() - screen_pos.x;

        if CAMERA_MOVE_BORDER.contains(&y_cursor_delta) {
            camera_transform.translation.y += CAMERA_SPEED * time.delta_seconds();
        } else if CAMERA_MOVE_BORDER.contains(&screen_pos.y) {
            camera_transform.translation.y -= CAMERA_SPEED * time.delta_seconds();
        } else if CAMERA_MOVE_BORDER.contains(&x_cursor_delta) {
            camera_transform.translation.x += CAMERA_SPEED * time.delta_seconds();
        } else if CAMERA_MOVE_BORDER.contains(&screen_pos.x) {
            camera_transform.translation.x -= CAMERA_SPEED * time.delta_seconds();
        }
    }


    if keys.pressed(W) {
        camera_transform.translation.y += CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(S) {
        camera_transform.translation.y -= CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(A) {
        camera_transform.translation.x -= CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(D) {
        camera_transform.translation.x += CAMERA_SPEED * time.delta_seconds();
    }
}
