use crate::KeyCode::*;
use crate::menu::*;
use crate::player::*;
use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system_set(
                SystemSet::on_update(World)
                    .with_system(camera_motion)
            )
            .add_system_set(
                SystemSet::on_exit(Combat)
                    .with_system(update_camera_position)
            )
            .add_system_set(
                SystemSet::on_enter(Menu)
                    .with_system(update_camera_in_menu.after(spawn_menu))
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

fn update_camera_position(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    player_query: Query<&Transform, (Without<Camera>, With<Player>)>,
) {
    let mut camera = camera_query.single_mut();
    let player = player_query.single();
    camera.translation = Vec3::new(player.translation.x, player.translation.y, camera.translation.z);
}

fn update_camera_in_menu(
    mut camera_query: Query<&mut Transform, UiCameraFilter>,
) {
    let mut camera = camera_query.single_mut();
    camera.translation = Vec3::new(10., -10., camera.translation.z);
}

fn camera_motion(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    windows: Res<Windows>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let window = windows.get_primary().unwrap();

    let can_move_top_y  = camera_transform.translation.y < 4.;
    let can_move_bot_y = camera_transform.translation.y > -20.;
    let can_move_left_x = camera_transform.translation.x > 1.;
    let can_move_right_x = camera_transform.translation.x < 13.;

    if let Some(screen_pos) = window.cursor_position() {
        let y_cursor_delta = window.height() - screen_pos.y;
        let x_cursor_delta = window.width() - screen_pos.x;

        if CAMERA_MOVE_BORDER.contains(&y_cursor_delta) && can_move_top_y {
            camera_transform.translation.y += CAMERA_SPEED * time.delta_seconds();
        } else if CAMERA_MOVE_BORDER.contains(&screen_pos.y) && can_move_bot_y {
            camera_transform.translation.y -= CAMERA_SPEED * time.delta_seconds();
        } else if CAMERA_MOVE_BORDER.contains(&x_cursor_delta) && can_move_right_x {
            camera_transform.translation.x += CAMERA_SPEED * time.delta_seconds();
        } else if CAMERA_MOVE_BORDER.contains(&screen_pos.x) && can_move_left_x {
            camera_transform.translation.x -= CAMERA_SPEED * time.delta_seconds();
        }
    }

    if keys.pressed(W) && can_move_top_y{
        camera_transform.translation.y += CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(S) && can_move_bot_y {
        camera_transform.translation.y -= CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(A) && can_move_left_x {
        camera_transform.translation.x -= CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(D) && can_move_right_x{
        camera_transform.translation.x += CAMERA_SPEED * time.delta_seconds();
    }
}
