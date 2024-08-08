pub(crate) mod prelude {
    pub(crate) use super::{Enemy, Enemies, take_damage};
}

use crate::prelude::*;

#[derive(Component)]
pub struct Enemy {
    is_deinit: bool,
    health: u32
}

#[derive(Reflect, Resource)]
pub struct Enemies {
    current_number: u32,
    max_number: u32,
    speed: f32,
    health: u32,
    distance: f32,
    max_range_spawn: I64Vec2,
    min_range_spawn: I64Vec2,
    timer_spawning: Timer,
}

impl Default for Enemies {
    fn default() -> Self {
        Self { current_number: 0, max_number: 0, timer_spawning: Timer::new(Duration::from_secs(1), TimerMode::Repeating), speed: 50f32, distance: 42f32, max_range_spawn: I64Vec2 {x: -80, y: 50}, min_range_spawn: I64Vec2 {x: -50, y: 20}, health: 100}
    }
}

pub fn plugin(
    app: &mut App
) {
    app.init_resource::<Enemies>();
    app.register_type::<Enemies>();
    //app.add_plugins(ResourceInspectorPlugin::<Enemies>::new());
    app.add_systems(Update, (update_position, spawning, update_gizmos));
}

pub fn init(
    mut cmd: Commands, 
    mut enemy_ref: ResMut<Enemies>, 
    position: Vec3
) {
    enemy_ref.current_number += 1;
    cmd
    .spawn(Enemy {is_deinit: false, health: enemy_ref.health})
    .insert(SpriteBundle { transform: Transform { translation: position + Vec3 {x: 30f32, y: 30f32, z: 0f32} / 2f32, scale: Vec3 {x: 30f32, y: 30f32, z: 0f32}, ..Default::default()}, sprite: Sprite { color: color_palette::ColorPalette::GRAY, ..Default::default()}, ..Default::default()});
}

fn deinit(
    cmd: &mut Commands, enemy: &mut Enemy, enemy_ref: &mut ResMut<Enemies>, entity: Entity
) {
    if enemy.is_deinit { return; }

    enemy.is_deinit = true;
    enemy_ref.current_number -= 1;
    cmd.entity(entity).despawn_recursive();
}

fn update_position(
    time: Res<Time>, res_enemies: ResMut<Enemies>, query_player: Query<&Transform, With<player::PlayerSprite>>, mut query_enemy: Query<&mut Transform, (With<Enemy>, Without<player::PlayerSprite>)>
) {
    for mut enemy_transform in query_enemy.iter_mut() {
        let player_position = query_player.single().translation;
        let enemy_position = enemy_transform.translation;
        let dir = (player_position - enemy_position).normalize();

        if enemy_position.distance(player_position) > res_enemies.distance {
            enemy_transform.translation += dir * res_enemies.speed * time.delta_seconds();
        }
    }
}

fn spawning(
    cmd: Commands, time: Res<Time>, mut res_enemies: ResMut<Enemies>
) {
    res_enemies.timer_spawning.tick(time.delta());
    if res_enemies.current_number < res_enemies.max_number && res_enemies.timer_spawning.finished() {
        let x_rand = rand::thread_rng().gen_range(res_enemies.max_range_spawn.x as i32..res_enemies.max_range_spawn.y as i32) as f32;
        let y_rand = rand::thread_rng().gen_range(res_enemies.min_range_spawn.x as i32..res_enemies.min_range_spawn.y as i32) as f32;

        init(cmd, res_enemies, Vec3 {x: x_rand, y: y_rand, ..Default::default()});
    }
}

pub fn take_damage(
    cmd: &mut Commands, bullet_entity: Entity, enemy_entity: Entity, enemy: &mut Enemy, enemy_ref: &mut ResMut<Enemies>
) -> bool {
    if let Some(new_value) = enemy.health.checked_sub(10) {
        cmd.entity(bullet_entity).despawn_recursive();
        enemy.health = new_value; 
        false
    }
    else {
        deinit(cmd, enemy, enemy_ref, enemy_entity);
        true
    }
}

fn update_gizmos(
    mut gizmos: Gizmos, res_enemies: ResMut<Enemies>
) {
    gizmos.rect_2d(Vec2::ZERO, 0f32, Vec2 {x: res_enemies.max_range_spawn.x as f32, y: res_enemies.max_range_spawn.y as f32}, color_palette::ColorPalette::WHITE);
}