pub(crate) mod prelude {
    pub(crate) use super::{
        Weapon, 
        WeaponType, 
        WeaponProjectileType, 
        WeaponHolder, 
        WeaponMuzzle, 
        Pistol, 
        Machinegun, 
        PistolProjectile, 
        MachinegunProjectile
    };
}

use crate::prelude::*;

#[derive(Reflect, Component)]
pub(crate) struct WeaponProjectile {
    weapon_projectile_type: WeaponProjectileType
}

#[derive(Reflect)]
pub(crate) enum WeaponProjectileType {
    PistolProjectile(PistolProjectile),    
    MachinegunProjectile(MachinegunProjectile)
}

#[derive(Clone, Reflect)]
pub(crate) struct PistolProjectile {
    pub(crate) timer_lifetime: Timer,
    pub(crate) force: f32,
    pub(crate) direction: Vec3
}

#[derive(Clone, Reflect)]
pub(crate) struct MachinegunProjectile {
    pub(crate) timer_lifetime: Timer,
    pub(crate) force: f32,
    pub(crate) direction: Vec3
}

impl WeaponProjectile {
    fn update_position(&mut self,
        cmd: &mut Commands,
        transform: &mut Transform,
        time: &Time,
        entity: Entity
    ) {  
        match &mut self.weapon_projectile_type {
            WeaponProjectileType::PistolProjectile(ammo) => {ammo.update_position(cmd, transform, time, entity);},
            WeaponProjectileType::MachinegunProjectile(ammo) => {ammo.update_position(cmd, transform, time, entity);}
        }
    }
}

impl WeaponProjectileType {
    fn spawn(&self,
        cmd: &mut Commands,
        position: Vec3,
        direction: Vec3
    ) {
        match self {
            WeaponProjectileType::PistolProjectile(projectile) => {projectile.spawn(cmd, position, direction)}
            WeaponProjectileType::MachinegunProjectile(projectile) => {projectile.spawn(cmd, position, direction)}
        }
    }
}

impl PistolProjectile {
    pub(crate) fn spawn(&self,
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

        let mut projectile = self.clone();
        projectile.direction = direction;

        let weapon_ammunition_component = WeaponProjectile {
            weapon_projectile_type: WeaponProjectileType::PistolProjectile(projectile)
        };

        cmd.entity(entity).insert(sprite_bundle).insert(weapon_ammunition_component).insert(Name::new("PistolAmmunition"));
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

impl MachinegunProjectile {
    pub(crate) fn spawn(&self,
        cmd: &mut Commands,
        position: Vec3,
        direction: Vec3
    ) {
        let entity = cmd.spawn(()).id();
        let sprite_bundle = SpriteBundle {
            transform: Transform {
                translation: position,
                rotation: Quat::from_axis_angle(Vec3::Z, direction.xy().to_angle()),
                scale: Vec3 {x: 20f32, y: 7f32, z: 0f32}
            },
            sprite: Sprite {
                color: color_palette::ColorPalette::RED,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let mut projectile = self.clone();
        projectile.direction = direction;

        let weapon_ammunition_component = WeaponProjectile {
            weapon_projectile_type: WeaponProjectileType::MachinegunProjectile(projectile)
        };

        cmd.entity(entity).insert(sprite_bundle).insert(weapon_ammunition_component).insert(Name::new("MachinegunAmmunition"));
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
    Pistol(Pistol),
    Machinegun(Machinegun),
    None,
}

#[derive(Reflect)]
pub(crate) struct Pistol {
    pub(crate) weapon_projectile: WeaponProjectileType,
    pub(crate) cooldown: Timer
}

#[derive(Reflect)]
pub(crate) struct Machinegun { 
    pub(crate) weapon_projectile: WeaponProjectileType,
    pub(crate) cooldown: Timer,
}

impl Pistol {
    pub(crate) fn spawn(self,
        cmd: &mut Commands
    ) -> Entity {
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
            weapon_type: WeaponType::Pistol(self), 
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

        return cmd.entity(weapon_holder_entity).add_child(weapon_entity).id();
    }

    pub(crate) fn update_position(&mut self,
        transform: &mut Transform,
        transform_player: &Transform
    ) {
        transform.translation = transform_player.translation;
    }

    pub(crate) fn shoot(&mut self,
        cmd: &mut Commands,
        time: &Time,
        mouse_button: &ButtonInput<MouseButton>,
        transform: (Vec3, Vec3),
    ) {
        self.cooldown.tick(time.delta());

        if mouse_button.just_pressed(MouseButton::Left) {  
            if self.cooldown.finished() {
                self.weapon_projectile.spawn(cmd, transform.0, transform.1);
                self.cooldown.reset();
            }
        } 
    }
}

impl Machinegun {
    pub(crate) fn spawn(self,
        cmd: &mut Commands
    ) -> Entity {
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
            weapon_type: WeaponType::Machinegun(self), 
            direction: Vec3::X,
        };
        
        let sprite_bundle = SpriteBundle {
            transform: Transform {
                translation: Vec3 {x: 30f32, y: 0f32, z: 0f32},
                rotation: Quat::from_axis_angle(Vec3::Z, Vec2::X.to_angle()),
                scale: Vec3 {x: 30f32, y: 20f32, z: 0f32}
            },
            sprite: Sprite {
                color: color_palette::ColorPalette::WHITE,
                ..Default::default()
            },
            ..Default::default()
        };
    
        let weapon_entity = cmd.spawn(weapon_component)
            .add_child(weapon_muzzle_entity)
            .insert(sprite_bundle)
            .insert(Name::new("Machinegun"))
        .id();

        return cmd.entity(weapon_holder_entity).add_child(weapon_entity).id();
    }

    pub(crate) fn update_position(&mut self,
        transform: &mut Transform,
        transform_player: &Transform
    ) {
        transform.translation = transform_player.translation;
    }

    pub(crate) fn shoot(&mut self,
        cmd: &mut Commands,
        time: &Time,
        mouse_button: &ButtonInput<MouseButton>,
        transform: (Vec3, Vec3),
    ) {
        self.cooldown.tick(time.delta());

        if mouse_button.pressed(MouseButton::Left) {  
            if self.cooldown.finished() {
                self.weapon_projectile.spawn(cmd, transform.0, transform.1);
                self.cooldown.reset();
            }
        } 
    }
}

impl Weapon {
    pub(crate) fn init(
        cmd: &mut Commands,
        weapon_type: WeaponType
    ) -> Option<Entity>{
        match weapon_type {
            WeaponType::Pistol(weapon) => {
                return Some(weapon.spawn(cmd));
            },
            WeaponType::Machinegun(weapon) => {
                return Some(weapon.spawn(cmd));
            },
            WeaponType::None => {return None;},
        }
    }

    pub(crate) fn update_position(&mut self,
        transform: &mut Transform,
        transform_player: &Transform
    ) {
        match &mut self.weapon_type {
            WeaponType::Pistol(weapon) => {
                weapon.update_position(transform, transform_player);
            },
            WeaponType::Machinegun(weapon) => {
                weapon.update_position(transform, transform_player);
            },
            _ => {}
        }
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

    pub(crate) fn make_shoot(&mut self,
        cmd: &mut Commands,
        time: &Time,
        mouse_button: &ButtonInput<MouseButton>,
        transform: (Vec3, Vec3),
    ) {
        match &mut self.weapon_type {
            WeaponType::Pistol(weapon) => {
                weapon.shoot(cmd, time, mouse_button, transform);
            },
            WeaponType::Machinegun(weapon) => {
                weapon.shoot(cmd, time, mouse_button, transform);
            },
            _ => {}
        }
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
    mut query: Query<(Entity, &mut Transform, &mut WeaponProjectile)>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut damage_object) in &mut query {
        damage_object.update_position(&mut cmd, transform.as_mut(), time.as_ref(), entity)
    }
}

fn update_weapon_position(
    mut query_weapon: Query<&mut Weapon>,
    mut query_weapon_holder: Query<&mut Transform, With<WeaponHolder>>,
    mut query_player: Query<&mut Transform, (With<player::PlayerSprite>, Without<WeaponHolder>)>
) {
    let mut weapon_holder = query_weapon_holder.single_mut();
    let mut weapon = query_weapon.single_mut();
    let mut player = query_player.single_mut();

    weapon.update_position(weapon_holder.as_mut(), player.as_mut())
}

fn update_weapon_rotation(
    mut query_weapon: Query<&mut Weapon>, 
    mut query_weapon_holder: Query<&mut Transform, With<WeaponHolder>>,
    query_player: Query<&Transform, (With<player::PlayerSprite>, Without<WeaponHolder>)>, 
    query_player_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>, 
    window: Query<&Window>
) {
    let mut weapon = query_weapon.single_mut();
    let mut wepon_holder = query_weapon_holder.single_mut();
    let player = query_player.single();
    let player_camera = query_player_camera.single();
    let win = window.single();

    weapon.update_rotation(wepon_holder.as_mut(), player, win, player_camera.0, player_camera.1);
}