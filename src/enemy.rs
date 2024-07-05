pub(crate) mod prelude {
    pub(crate) use super::Enemy;
}

use crate::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Reflect, Resource)]
pub struct Enemies {
    timer_spawning: Timer,
    speed: f32,
    distance: f32,
    max_range_spawn: I64Vec2,
    min_range_spawn: I64Vec2,

}

impl Default for Enemies {
    fn default() -> Self {
        Self { timer_spawning: Timer::new(Duration::from_secs(1), TimerMode::Repeating), speed: 50f32, distance: 42f32, max_range_spawn: I64Vec2 {x: -80, y: 50}, min_range_spawn: I64Vec2 {x: -50, y: 20}}
    }
}

trait NewTrait {
    fn gen_range_beyond_range(self, goal_range: Range<i32> , beyond_range: Range<i32>) -> i32;
}

impl NewTrait for ThreadRng {
    fn gen_range_beyond_range(self, goal_range: Range<i32> , beyond_range: Range<i32>) -> i32 {
        let mut rand = self;
        let rand_number1 = rand.gen_range(goal_range.start..beyond_range.start);
        let rand_number2= rand.gen_range(beyond_range.end..goal_range.end);
        let rand_number = if rand.gen_range(0..=1) == 0 {rand_number1} else {rand_number2};
        rand_number
    }
}

impl Enemy {
    pub fn main(app: &mut App) {
        app.init_resource::<Enemies>();
        app.register_type::<Enemies>();
        app.add_plugins(ResourceInspectorPlugin::<Enemies>::new());
        app.add_systems(Update, (Enemy::update_position, Enemy::spawning, Enemy::update_gizmos));
    }

    pub fn init(mut commands: Commands, position: Vec3) {
        commands
        .spawn(Enemy)
        .insert(SpriteBundle { transform: Transform { translation: position + Vec3 {x: 30f32, y: 30f32, z: 0f32} / 2f32, scale: Vec3 {x: 30f32, y: 30f32, z: 0f32}, ..Default::default()}, sprite: Sprite { color: Color::hex("#ff4f4f").unwrap(), ..Default::default()}, ..Default::default()});
    }

    pub fn update_position(time: Res<Time>, res_enemies: ResMut<Enemies>, query_player: Query<&Transform, With<PlayerSprite>>, mut query_enemy: Query<&mut Transform, (With<Enemy>, Without<PlayerSprite>)>) {
        for mut enemy_transform in query_enemy.iter_mut() {
            let player_position = query_player.single().translation;
            let enemy_position = enemy_transform.translation;
            let dir = (player_position - enemy_position).normalize();

            if enemy_position.distance(player_position) > res_enemies.distance {
                enemy_transform.translation += dir * res_enemies.speed * time.delta_seconds();
            }
        }
    }

    pub fn spawning(cmd: Commands, time: Res<Time>, mut res_enemies: ResMut<Enemies>) {
        res_enemies.timer_spawning.tick(time.delta());
        if res_enemies.timer_spawning.finished() {
            // let x_rand = rand::thread_rng().gen_range_beyond_range(res_enemies.max_range_spawn.x as i32..res_enemies.max_range_spawn.y as i32, res_enemies.min_range_spawn.x as i32..res_enemies.min_range_spawn.y as i32) as f32;
            // let y_rand = rand::thread_rng().gen_range_beyond_range(res_enemies.max_range_spawn.x as i32..res_enemies.max_range_spawn.y as i32, res_enemies.min_range_spawn.x as i32..res_enemies.min_range_spawn.y as i32) as f32;

            let x_rand = rand::thread_rng().gen_range(res_enemies.max_range_spawn.x as i32..res_enemies.max_range_spawn.y as i32) as f32;
            let y_rand = rand::thread_rng().gen_range(res_enemies.min_range_spawn.x as i32..res_enemies.min_range_spawn.y as i32) as f32;
            println!("x_rand: {}, y_rand: {}", x_rand, y_rand);

            Enemy::init(cmd, Vec3 {x: x_rand, y: y_rand, ..Default::default()});
        }
    }

    pub fn update_gizmos(mut gizmos: Gizmos, res_enemies: ResMut<Enemies>) {
        gizmos.circle_2d(Vec2::ZERO, 10f32, Color::RED);
        gizmos.rect_2d(Vec2::ZERO, 0f32, Vec2 {x: res_enemies.max_range_spawn.x as f32, y: res_enemies.max_range_spawn.y as f32}, Color::WHITE);
        gizmos.rect_2d(Vec2::ZERO, 0f32, Vec2 {x: res_enemies.min_range_spawn.x as f32, y: res_enemies.min_range_spawn.y as f32}, Color::GRAY);
    }
}