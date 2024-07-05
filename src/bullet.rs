pub(crate) mod prelude {
    pub(crate) use super::{Bullet, Bullets};
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

impl Bullet {
    pub fn main(app: &mut App) {
        app.register_type::<Bullet>();
        app.init_resource::<Bullets>();
        app.register_type::<Bullets>();
        app.add_plugins(ResourceInspectorPlugin::<Bullets>::new());
        app.add_systems(Update, (Bullet::update_position, Bullet::update_intersection));
    }

    pub fn update_position(
        time: Res<Time>, 
        mut commands: Commands,
        mut query_child: Query<(Entity, &mut Transform, &mut Bullet)>) {
        for (entity, mut transform, mut bullet) in &mut query_child {
            transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
            bullet.timer_lifetime.tick(time.delta());

            if bullet.timer_lifetime.finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    } 

    pub fn update_intersection(mut cmd: Commands, query_enemy: Query<(Entity, &Transform), (With<Enemy>, Without<Bullet>)>, query_bullet: Query<&Transform, With<Bullet>>) {
        for enemy in query_enemy.iter(){
            for bullet in query_bullet.iter() {
                let b_aabb = Aabb2d {min: Vec2 { x: bullet.translation.x - bullet.scale.x / 2f32, y: bullet.translation.y - bullet.scale.y / 2f32}, max: Vec2 { x: bullet.translation.x + bullet.scale.x / 2f32, y: bullet.translation.y + bullet.scale.y / 2f32}};
                let e_aabb = Aabb2d {min: Vec2 { x: enemy.1.translation.x - enemy.1.scale.x / 2f32, y: enemy.1.translation.y - enemy.1.scale.y / 2f32}, max: Vec2 { x: enemy.1.translation.x + enemy.1.scale.x / 2f32, y: enemy.1.translation.y + enemy.1.scale.y / 2f32 }};
                if b_aabb.intersects(&e_aabb){
                    cmd.entity(enemy.0).despawn_recursive();
                    break;
                }
            }
        }
    }

    pub fn init(mut commands: Commands, res_bullet: Res<Bullets>, translation: Vec3, direction: Vec3) {
        let a = commands.spawn(()).id();
        commands.entity(a).insert(SpriteBundle { transform: Transform { translation: translation, rotation: Quat::from_axis_angle(Vec3::Z, direction.xy().to_angle()), scale: Vec3 {x: 30f32, y: 15f32, z: 0f32} }, sprite: Sprite { color: Color::hex("#ffaa71").unwrap(), ..Default::default()}, ..Default::default()})
        .insert(Bullet {timer_lifetime: Timer::new(Duration::from_secs(res_bullet.lifetime), TimerMode::Once), speed: res_bullet.speed, direction: direction});
    }
}