pub(crate) mod prelude {
    pub(crate) use super::{Player, PlayerSprite};
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
    let player_entity = cmd.spawn(Player {
        speed: 100f32, 
        direction_y: Vec3::ZERO, 
        keyboard_direction: Vec3::ZERO,
        position: Vec3::ZERO,
        rotation: Vec3::ZERO
    }).id();

    let weapon_entity = weapon::Weapon::init(&mut cmd, super::weapon::WeaponType::Pistol).unwrap();

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
        PlayerSprite,
        Name::new("Sprite")
    )).id();

    let transform_bundle = (GlobalTransform::default(), Transform::default(), InheritedVisibility::default());

    cmd.entity(player_entity)
    .add_child(weapon_entity)
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

fn update_shooting(
    mut cmd: Commands, 
    query_weapon_muzzle: Query<&GlobalTransform, With<weapon::WeaponMuzzle>>,
    query_weapom: Query<&weapon::Weapon>,
    mouse_button: Res<ButtonInput<MouseButton>>
) {
    let weapon_muzzle = query_weapon_muzzle.single();
    let weapon = query_weapom.single();

    if mouse_button.just_pressed(MouseButton::Right) {  
        weapon::MachinegunAmmunition::init(&mut cmd, weapon_muzzle.translation(), weapon.direction);
    } else if mouse_button.just_pressed(MouseButton::Left) {
        weapon::PistolAmmunition::init(&mut cmd, weapon_muzzle.translation(), weapon.direction);
    }
}