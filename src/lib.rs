pub mod observers;
pub mod components;
pub mod bundles;
pub mod render;
mod despawn;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::StatBarsPlugin;
    pub use crate::bundles::*;
    pub use crate::observers::component_observer;
}

use bevy::prelude::*;

pub struct StatBarsPlugin;

impl Plugin for StatBarsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(render::RenderStatBarsPlugin)
        .add_plugin(observers::StatBarObserverPlugin)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            despawn::despawn_if_subject_not_found
        );
    }
}