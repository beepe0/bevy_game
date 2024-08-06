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
    let player_entity = cmd.spawn(Player {
        speed: 100f32, 
        direction_y: Vec3::ZERO, 
        keyboard_direction: Vec3::ZERO,
        position: Vec3::ZERO,
        rotation: Vec3::ZERO
    }).id();

    let weapon_entity = weapon::Weapon::init(&mut cmd, weapon::WeaponType::Pistol(weapon::Pistol {weapon_projectile: {
        weapon::WeaponProjectileType::MachinegunProjectile(weapon::MachinegunProjectile {
            timer_lifetime: Timer::new(Duration::from_secs(2), TimerMode::Once),
            force: 700f32,
            direction: Vec3::X
        })
    }})).unwrap();

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

    player.keyboard_direction.y = (keyboard.pressed(KeyCode::KeyW) as i32 as f32) - (keyboard.pressed(KeyCode::KeyS) as i32 as f32);
    player.keyboard_direction.x = keyboard.pressed(KeyCode::KeyD) as i32 as f32 - (keyboard.pressed(KeyCode::KeyA) as i32 as f32);
} 

fn update_shooting(
    mut cmd: Commands, 
    mut query_weapon: Query<&mut weapon::Weapon>,
    query_weapon_muzzle: Query<&GlobalTransform, With<weapon::WeaponMuzzle>>,
    mouse_button: Res<ButtonInput<MouseButton>>
) {
    let weapon_muzzle = query_weapon_muzzle.single();
    let mut weapon = query_weapon.single_mut();

    if mouse_button.just_pressed(MouseButton::Left) {  
        let dir = weapon.direction;
        weapon.make_shoot(&mut cmd, weapon_muzzle.translation(), dir);
        //weapon::MachinegunAmmunition::spawn(&mut cmd, weapon_muzzle.translation(), weapon.direction);
    } else if mouse_button.just_pressed(MouseButton::Right) {
        //weapon::PistolAmmunition::spawn(&mut cmd, weapon_muzzle.translation(), weapon.direction);
    }
}