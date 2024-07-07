mod prelude;
mod inspector;
mod player;
mod bullet;
mod enemy;
mod color_palette;

use crate::prelude::*;

pub fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(RenderPlugin { render_creation: RenderCreation::Automatic(WgpuSettings {backends: Some(Backends::VULKAN), ..Default::default()}), ..Default::default()}))    
    .add_plugins(inspector::plugin)
    .add_plugins(player::plugin)
    .add_plugins(enemy::plugin)
    .add_plugins(bullet::plugin)
    .run();
}