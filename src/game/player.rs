pub(crate) mod prelude {
    pub(crate) use super::PlayerSprite;
}

use crate::prelude::*;

#[derive(Reflect, Component)]
pub(crate) struct Player {
    motion_acceleration: f32,
    direction_view: Vec3,
    keyboard_direction: Vec3,
    position: Vec3,
    rotation: Vec3,
    weapons: Vec<Option<Entity>>
}

#[derive(Component)]
pub(crate) struct PlayerSprite;

impl Player {
    fn swap_weapon() {

    }
}

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
    let player = Player {
        motion_acceleration: 100f32, 
        direction_view: Vec3::ZERO, 
        keyboard_direction: Vec3::ZERO,
        position: Vec3::ZERO,
        rotation: Vec3::ZERO,
        weapons: Vec::new(),
    };

    let pistol_entity = weapon::Weapon::init(&mut cmd, weapon::WeaponType::Pistol(weapon::Pistol {weapon_projectile: {
        weapon::WeaponProjectileType::PistolProjectile(weapon::PistolProjectile {
            timer_lifetime: Timer::new(Duration::from_secs(2), TimerMode::Once),
            force: 1000f32,
            direction: Vec3::X
        })
    }, cooldown: Timer::new(Duration::from_millis(10), TimerMode::Once)})).unwrap();

    // let machinegun_entity = weapon::Weapon::init(&mut cmd, weapon::WeaponType::Machinegun(weapon::Machinegun { weapon_projectile: {
    //     weapon::WeaponProjectileType::MachinegunProjectile(weapon::MachinegunProjectile { 
    //         timer_lifetime: Timer::new(Duration::from_secs(2), TimerMode::Once), 
    //         force: 2000f32, 
    //         direction: Vec3::X 
    //     })
    // }, cooldown: Timer::new(Duration::from_millis(100), TimerMode::Once)})).unwrap();

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

    cmd.spawn(player)
    .add_child(camera_entity)
    .add_child(sprite_entity)
    .insert(transform_bundle)
    .insert(Name::new("Player"));
}

fn update_position(
    mut query_child: Query<&mut Transform, With<PlayerSprite>>, 
    mut query_parent: Query<&mut Player>, 
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) { 
    let mut transform = query_child.single_mut();
    let mut player = query_parent.single_mut();

    transform.translation += (player.keyboard_direction) * player.motion_acceleration * time.delta_seconds();
    player.keyboard_direction = Vec3::ZERO;

    player.keyboard_direction.y = (keyboard.pressed(KeyCode::KeyW) as i32 as f32) - (keyboard.pressed(KeyCode::KeyS) as i32 as f32);
    player.keyboard_direction.x = keyboard.pressed(KeyCode::KeyD) as i32 as f32 - (keyboard.pressed(KeyCode::KeyA) as i32 as f32);
} 

fn update_shooting(
    mut cmd: Commands, 
    mut query_weapon: Query<&mut weapon::Weapon>,
    query_weapon_muzzle: Query<&GlobalTransform, With<weapon::WeaponMuzzle>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let mut weapon = query_weapon.single_mut();
    let weapon_muzzle = query_weapon_muzzle.single();
    let direction = weapon.direction;

    weapon.make_shoot(&mut cmd, time.as_ref(), mouse_button.as_ref(), (weapon_muzzle.translation(), direction));    
}