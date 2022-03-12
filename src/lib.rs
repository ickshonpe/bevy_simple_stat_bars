pub mod observers;
pub mod display;
pub mod components;
pub mod chase;
pub mod builder;
pub mod bundles;
pub mod render;

pub(crate) use bevy::math::vec2;
pub(crate) use bevy::prelude::*;
pub(crate) use components::*;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::display::*;
    pub use crate::chase::*;
    pub use crate::StatusBarsPlugin;
}

const DEFAULT_BAR_Z_DEPTH: f32 = 950.0;

pub struct StatusBarsPlugin;

impl Plugin for StatusBarsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(chase::StatusBarChasePlugin)
        .add_plugin(render::RenderStatusBarsPlugin)
        ;
    }
}