pub(crate) mod prelude {
    pub(crate) use super::Weapon;
}

use crate::prelude::*;

pub(crate) trait FirearmsWeapon {
    fn make_shoot();
    fn take_cooldown();
}

pub(crate) trait MeleeWeapon {
    fn make_punch();
}

pub(crate) trait ProjectileWeapon {
    fn make_throw();
}
#[derive(Reflect, Component)]
pub(crate) enum DamageObject {
    PistolAmmunition(PistolAmmunition),    
}

#[derive(Reflect)]
struct PistolAmmunition {
    timer: Timer,
    force: f32,
    direction: Vec3
}

impl PistolAmmunition {
    pub(crate) fn init(
        mut cmd: Commands,
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
        let damage_object = DamageObject::PistolAmmunition(PistolAmmunition {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            force: 10f32,
            direction: direction
        });
        cmd.entity(entity).insert(sprite_bundle).insert(damage_object);
    }

    pub(crate) fn update_position(

    ){

    }
}

#[derive(Reflect, Component)]
pub(crate) enum Weapon {
    Pistol(Pistol),    
}

#[derive(Reflect)]
struct Pistol {}

impl FirearmsWeapon for Pistol {
    fn make_shoot() {
        todo!()
    }

    fn take_cooldown() {
        todo!()
    }
}

pub(crate) fn plugin(
    app: &mut App
) {

}

fn update_position(
    time: Res<Time>,
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Transform, &mut DamageObject)>
) {
    for (entity, mut transform, mut damage_object) in &mut query {
        match damage_object {
            DamageObject::PistolAmmunition(ammo) => {}
        }
    }
}

fn update_intersection_bullets() {

}