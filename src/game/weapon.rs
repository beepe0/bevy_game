pub(crate) mod prelude {
    pub(crate) use super::{Weapon, WeaponHolder, WeaponMuzzle, PistolAmmunition, MachinegunAmmunition};
}

use crate::prelude::*;

// pub(crate) trait FirearmsWeapon {
//     fn make_shoot();
//     fn take_reload();
// }

// pub(crate) trait MeleeWeapon {
//     fn make_stroke();
// }

// pub(crate) trait ProjectileWeapon {
//     fn make_throw();
// }
#[derive(Reflect, Component)]
pub(crate) enum WeaponAmmunition {
    PistolAmmunition(PistolAmmunition),    
    MachinegunAmmunition(MachinegunAmmunition)
}

#[derive(Reflect)]
pub(crate) struct PistolAmmunition {
    timer_lifetime: Timer,
    force: f32,
    direction: Vec3
}

#[derive(Reflect)]
pub(crate) struct MachinegunAmmunition {
    timer_lifetime: Timer,
    force: f32,
    direction: Vec3
}

impl WeaponAmmunition {
    fn update_position(&mut self,
        cmd: &mut Commands,
        transform: &mut Transform,
        time: &Time,
        entity: Entity
    ) {  
        match self {
            WeaponAmmunition::PistolAmmunition(ammo) => {ammo.update_position(cmd, transform, time, entity);},
            WeaponAmmunition::MachinegunAmmunition(ammo) => {ammo.update_position(cmd, transform, time, entity);}
        }
    }
}
impl PistolAmmunition {
    pub(crate) fn init(
        cmd: &mut Commands,
        position: Vec3,
        direction: Vec3
    ) {
        let entity = cmd.spawn(()).id();
        let sprite_bundle = SpriteBundle {
            transform: Transform {
                translation: position,
                rotation: Quat::from_axis_angle(Vec3::Z, direction.xy().to_angle()),
                scale: Vec3 {x: 30f32, y: 10f32, z: 0f32}
            },
            sprite: Sprite {
                color: color_palette::ColorPalette::WHITE,
                ..Default::default()
            },
            ..Default::default()
        };
        let damage_object = WeaponAmmunition::PistolAmmunition(PistolAmmunition {
            timer_lifetime: Timer::new(Duration::from_secs(2), TimerMode::Once),
            force: 300f32,
            direction: direction
        });
        cmd.entity(entity).insert(sprite_bundle).insert(damage_object).insert(Name::new("PistolAmmunition"));
    }

    pub(crate) fn update_position(&mut self,
        cmd: &mut Commands,
        transform: &mut Transform,
        time: &Time,
        entity: Entity
    ){
        transform.translation += self.direction * self.force * time.delta_seconds();
        self.timer_lifetime.tick(time.delta());

        if self.timer_lifetime.just_finished() { 
            cmd.entity(entity).despawn_recursive();
        }
    }
}

impl MachinegunAmmunition {
    pub(crate) fn init(
        cmd: &mut Commands,
        position: Vec3,
        direction: Vec3
    ) {
        let entity = cmd.spawn(()).id();
        let sprite_bundle = SpriteBundle {
            transform: Transform {
                translation: position,
                rotation: Quat::from_axis_angle(Vec3::Z, direction.xy().to_angle()),
                scale: Vec3 {x: 50f32, y: 10f32, z: 0f32}
            },
            sprite: Sprite {
                color: color_palette::ColorPalette::RED,
                ..Default::default()
            },
            ..Default::default()
        };
        let damage_object = WeaponAmmunition::PistolAmmunition(PistolAmmunition {
            timer_lifetime: Timer::new(Duration::from_secs(2), TimerMode::Once),
            force: 600f32,
            direction: direction
        });
        cmd.entity(entity).insert(sprite_bundle).insert(damage_object).insert(Name::new("MachinegunAmmunition"));
    }

    pub(crate) fn update_position(&mut self,
        cmd: &mut Commands,
        transform: &mut Transform,
        time: &Time,
        entity: Entity
    ){
        transform.translation += self.direction * self.force * time.delta_seconds();
        self.timer_lifetime.tick(time.delta());

        if self.timer_lifetime.just_finished() { 
            cmd.entity(entity).despawn_recursive();
        }
    }
}
#[derive(Component)]
pub(crate) struct WeaponHolder;

#[derive(Component)]
pub(crate) struct WeaponMuzzle;

#[derive(Reflect, Component)]
pub(crate) struct Weapon {
    pub(crate) weapon_type: WeaponType,
    pub(crate) direction: Vec3,    
}

#[derive(Reflect)]
pub(crate) enum WeaponType {
    Pistol,
    None,
}

impl Weapon {
    pub(crate) fn init(
        cmd: &mut Commands,
        weapon_type: WeaponType
    ) -> Option<Entity>{
        match weapon_type {
            WeaponType::Pistol => {
                let weapon_holder_entity = cmd.spawn((
                    weapon::WeaponHolder, 
                    GlobalTransform::default(), 
                    Transform::default(), 
                    InheritedVisibility::default(), 
                    Name::new("WeaponHolder")
                )).id();
                
                let weapon_muzzle_entity = cmd.spawn((
                    WeaponMuzzle, 
                    GlobalTransform::default(), 
                    Transform {
                        translation: Vec3 {x: 1f32, y: 0f32, z: 0f32},
                        ..Transform::default()
                    }, 
                    Name::new("WeaponMuzzle")
                )).id();
                
                let weapon_component = Weapon { 
                    weapon_type: WeaponType::Pistol, 
                    direction: Vec3::X,
                };
                
                let sprite_bundle = SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {x: 30f32, y: 0f32, z: 0f32},
                        rotation: Quat::from_axis_angle(Vec3::Z, Vec2::X.to_angle()),
                        scale: Vec3 {x: 40f32, y: 10f32, z: 0f32}
                    },
                    sprite: Sprite {
                        color: color_palette::ColorPalette::GRAY,
                        ..Default::default()
                    },
                    ..Default::default()
                };
            
                let weapon_entity = cmd.spawn(weapon_component)
                    .add_child(weapon_muzzle_entity)
                    .insert(sprite_bundle)
                    .insert(Name::new("Pistol"))
                .id();

                return Some(cmd.entity(weapon_holder_entity).add_child(weapon_entity).id());
            },
            WeaponType::None => {return None;},
        }
    }

    pub(crate) fn update_position(&mut self,
        transform: &mut Transform,
        transform_player: &mut Transform
    ) {
        transform.translation = transform_player.translation;
    }

    pub(crate) fn update_rotation(&mut self,
        weapon_transform: &mut Transform,
        player_transform: &Transform,
        canvas: &Window,
        camera: &Camera, 
        camera_transform: &GlobalTransform
    ) {
        if let Some(mouse_position) = canvas.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
            let direction = (mouse_position - player_transform.translation.xy()).normalize();
            let angle = direction.to_angle();

            self.direction = Vec3 {x: direction.x, y: direction.y, z: 0f32};
            weapon_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
        }
    }

    fn make_shoot(&mut self,
        cmd: &mut Commands,
        position: Vec3,
        direction: Vec3
    ) {
        PistolAmmunition::init(cmd, position, direction);
    }

    fn take_reload() {
        todo!()
    }
}

pub(crate) fn plugin(
    app: &mut App
) {
    app.add_systems(Update, (update_ammunition_position, update_weapon_position, update_weapon_rotation));
}

fn update_ammunition_position(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Transform, &mut WeaponAmmunition)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut damage_object) in &mut query {
        damage_object.update_position(&mut cmd, transform.as_mut(), time.as_ref(), entity)
    }
}

fn update_weapon_position(
    mut query_weapon_holder: Query<&mut Transform, With<WeaponHolder>>,
    mut query_weapon: Query<&mut Weapon>,
    mut query_player: Query<&mut Transform, (With<player::PlayerSprite>, Without<WeaponHolder>)>
) {
    let mut weapon_holder = query_weapon_holder.single_mut();
    let mut weapon = query_weapon.single_mut();
    let mut player = query_player.single_mut();

    weapon.update_position(weapon_holder.as_mut(), player.as_mut())
}

fn update_weapon_rotation(
    mut query_weapon: Query<(&mut Transform, &mut Weapon)>, 
    mut query_player: Query<&mut Transform, (With<player::PlayerSprite>, Without<Weapon>)>, 
    query_player_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, 
    window: Query<&Window>
) {
    let mut weapon = query_weapon.single_mut();
    let player = query_player.single_mut();
    let player_camera = query_player_camera.single();
    let win = window.single();

    weapon.1.update_rotation(weapon.0.as_mut(), player.as_ref(), win, player_camera.0, player_camera.1);
}