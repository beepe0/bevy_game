mod prelude;
mod inspector;
mod game;

use crate::prelude::*;

pub fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(RenderPlugin { render_creation: RenderCreation::Automatic(WgpuSettings {backends: Some(Backends::VULKAN), ..Default::default()}), ..Default::default()}))    
    .add_plugins(inspector::plugin)
    .add_plugins(game::player::plugin)
    .add_plugins(game::enemy::plugin)
    .add_plugins(game::weapon::plugin)
    .run();
}