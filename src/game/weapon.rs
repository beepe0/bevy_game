pub(crate) mod prelude {
    pub(crate) use super::{Weapon, Pistol, PistolAmmunition, MachinegunAmmunition};
}

use bevy::math::VectorSpace;

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

#[derive(Reflect, Component)]
pub(crate) enum Weapon {
    Pistol(Pistol),    
}

#[derive(Reflect)]
pub(crate) struct Pistol;

impl Pistol {
    pub(crate) fn init(
        cmd: &mut Commands,
        position: Vec3,
        direction: Vec3
    ) -> Entity{
        let entity = cmd.spawn(()).id();
        let sprite_bundle = SpriteBundle {
            transform: Transform {
                translation: position,
                rotation: Quat::from_axis_angle(Vec3::Z, direction.xy().to_angle()),
                scale: Vec3 {x: 40f32, y: 10f32, z: 0f32}
            },
            sprite: Sprite {
                color: color_palette::ColorPalette::GRAY,
                ..Default::default()
            },
            ..Default::default()
        };
        let weapon = Weapon::Pistol(Pistol);
        cmd.entity(entity).insert(sprite_bundle).insert(weapon).insert(Name::new("Pistol"));
        entity
    }

    pub(crate) fn update_position() {

    }

    pub(crate) fn update_rotation() {

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

) {

}

fn update_weapon_rotation(

) {

}