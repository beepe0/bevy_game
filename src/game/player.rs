pub(crate) mod prelude {
    pub(crate) use super::PlayerSprite;
}

use crate::prelude::*;

#[derive(Reflect, Component)]
pub(crate) struct Player {
    speed: f32,
    direction_y: Vec3,
    keyboard_direction: Vec3,
    position: Vec3,
    rotation: Vec3,
}

#[derive(Component)]
pub(crate) struct PlayerSprite;

pub fn plugin(
    app: &mut App
) {
    app.register_type::<Player>();
    app.add_systems(Startup, startup);
    app.add_systems(Update, (update_position, update_shooting));
}

fn startup(
    mut cmd: Commands,
) {
    let player_component = Player {
        speed: 100f32, 
        direction_y: Vec3::ZERO, 
        keyboard_direction: Vec3::ZERO,
        position: Vec3::ZERO,
        rotation: Vec3::ZERO
    };

    let weapon_component = weapon::Pistol::init(&mut cmd, Vec3::ZERO, Vec3::ZERO);

    let camera_entity = cmd.spawn(
        Camera2dBundle {
            camera: Camera { 
                clear_color: ClearColorConfig::Custom(color_palette::ColorPalette::DARKGRAY), ..Default::default() 
            }, 
            ..Default::default()},
        ).id();

    let sprite_entity = cmd.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3 {x: 1f32, y: 1f32, z: 1f32},
                ..Default::default()
            },
          sprite: Sprite {
                color: color_palette::ColorPalette::ORANGE, 
                custom_size: Some(Vec2::ONE * 20f32), 
                ..Default::default()},
            ..Default::default()
        },
        )).insert(PlayerSprite).id();

    let transform_bundle = (GlobalTransform::default(), Transform::default(), InheritedVisibility::default());

    cmd.spawn(player_component)
    .add_child(weapon_component)
    .add_child(camera_entity)
    .add_child(sprite_entity)
    .insert(transform_bundle)
    .insert(Name::new("Player"));
}

fn update_position(
    time: Res<Time>, 
    mut query_child: Query<&mut Transform, With<PlayerSprite>>, 
    mut query_parent: Query<&mut Player>, 
    keyboard: Res<ButtonInput<KeyCode>>
) { 
    let mut transform = query_child.single_mut();
    let mut player = query_parent.single_mut();

    transform.translation += (player.keyboard_direction) * player.speed * time.delta_seconds();
    player.keyboard_direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { player.keyboard_direction.y = 1f32; }
    if keyboard.pressed(KeyCode::KeyS) { player.keyboard_direction.y = -1f32; }
    if keyboard.pressed(KeyCode::KeyD) { player.keyboard_direction.x = 1f32; }
    if keyboard.pressed(KeyCode::KeyA) { player.keyboard_direction.x = -1f32; }
} 

// fn update_rotation(
//     mut query_parent: Query<&mut Player>, 
//     mut query_child_sprite: Query<&mut Transform, With<PlayerSprite>>, 
//     query_child_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, 
//     window: Query<&Window>
// ) {
//     let mut player = query_parent.single_mut();
//     let mut transform = query_child_sprite.single_mut();

//     let win = window.single();
//     let (cam, cam_transform) = query_child_camera.single();

//     if let Some(mouse_position) = win.cursor_position().and_then(|cursor| cam.viewport_to_world_2d(cam_transform, cursor)) {
//         let v = (mouse_position - transform.translation.xy()).normalize();
//         let angle = v.normalize().to_angle();
//         player.direction_y = Vec3 {x: v.x, y: v.y, z: 0f32};
//         transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
//     }
// }

fn update_shooting(
    mut cmd: Commands, 
    query_child: Query<&mut Transform, With<PlayerSprite>>, 
    query_parent: Query<&mut Player>, 
    mouse_button: Res<ButtonInput<MouseButton>>
) {
    let transform = query_child.single();
    let player = query_parent.single();

    if mouse_button.just_pressed(MouseButton::Right) {  
        weapon::MachinegunAmmunition::init(&mut cmd, transform.translation, player.direction_y);
    } else if mouse_button.just_pressed(MouseButton::Left) {
        weapon::PistolAmmunition::init(&mut cmd, transform.translation, player.direction_y);
    }
}