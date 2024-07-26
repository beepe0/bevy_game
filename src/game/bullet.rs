pub(crate) mod prelude {
    pub(crate) use super::{Bullets, init};
}

use crate::prelude::*;

#[derive(Reflect, Component)]
pub struct Bullet {
    timer_lifetime: Timer,
    speed: f32,
    direction: Vec3
}

#[derive(Reflect, Resource)]
pub struct Bullets {
    lifetime: u64,
    speed: f32,
}

impl Default for Bullets {
    fn default() -> Self {
        Self { speed: 250f32, lifetime: 2}
    }
}

pub fn plugin(
    app: &mut App
) {
    app.register_type::<Bullet>();
    app.init_resource::<Bullets>();
    app.register_type::<Bullets>();
    app.add_plugins(ResourceInspectorPlugin::<Bullets>::new());
    app.add_systems(Update, (update_position, update_intersection));
}

fn update_position(
    time: Res<Time>, 
    mut commands: Commands,
    mut query_child: Query<(Entity, &mut Transform, &mut Bullet)>
) {
    for (entity, mut transform, mut bullet) in &mut query_child {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
        bullet.timer_lifetime.tick(time.delta());

        if bullet.timer_lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
} 

fn update_intersection(
    mut cmd: Commands, 
    mut enemy_ref: ResMut<enemy::Enemies>, 
    mut query_enemy: Query<(Entity, &Transform, &mut enemy::Enemy), (With<enemy::Enemy>, Without<Bullet>)>, 
    query_bullet: Query<(Entity, &Transform), With<Bullet>>
) {
    'bullet_loop: for bullet in query_bullet.iter() {
        for mut enemy in query_enemy.iter_mut(){
            let b_aabb = Aabb2d {min: Vec2 { x: bullet.1.translation.x - bullet.1.scale.x / 2f32, y: bullet.1.translation.y - bullet.1.scale.y / 2f32}, max: Vec2 { x: bullet.1.translation.x + bullet.1.scale.x / 2f32, y: bullet.1.translation.y + bullet.1.scale.y / 2f32}};
            let e_aabb = Aabb2d {min: Vec2 { x: enemy.1.translation.x - enemy.1.scale.x / 2f32, y: enemy.1.translation.y - enemy.1.scale.y / 2f32}, max: Vec2 { x: enemy.1.translation.x + enemy.1.scale.x / 2f32, y: enemy.1.translation.y + enemy.1.scale.y / 2f32 }};
            if b_aabb.intersects(&e_aabb){
                if enemy::take_damage(&mut cmd, bullet.0, enemy.0, &mut enemy.2, &mut enemy_ref) {break};
                break 'bullet_loop;
            }
        }      
    }
}

pub(crate) fn init(
    mut commands: Commands, 
    res_bullet: Res<Bullets>, 
    translation: Vec3, 
    direction: Vec3
) {
    let entity = commands.spawn(()).id();
    commands.entity(entity).insert(
        SpriteBundle { 
            transform: Transform 
            { 
                translation: translation, 
                rotation: Quat::from_axis_angle(Vec3::Z, direction.xy().to_angle()), 
                scale: Vec3 {x: 30f32, y: 15f32, z: 0f32} }, 
                sprite: Sprite { 
                    color: color_palette::ColorPalette::WHITE, 
                    ..Default::default()
                }, 
                ..Default::default()
            })
    .insert(Bullet {timer_lifetime: Timer::new(Duration::from_secs(res_bullet.lifetime), TimerMode::Once), speed: res_bullet.speed, direction: direction});
}