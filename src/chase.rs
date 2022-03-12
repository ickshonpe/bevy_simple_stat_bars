use bevy::prelude::*;

// more limited than the Bevy transform propagation
// no recursive propagation

#[derive(Copy, Clone)]
#[derive(Component)]
pub struct InheritTranslation {
    pub target: Entity,
    pub displacement: Vec3,
}

fn inherit_translation(
    targets: Query<&GlobalTransform, Without<InheritTranslation>>,
    mut followers: Query<(&Transform, &mut GlobalTransform, &InheritTranslation)>,
) {
    followers.for_each_mut(|(transform, mut global_transform, inheritor)| {
        if let Ok(target_transform) = targets.get(inheritor.target) {
            global_transform.translation = transform.translation + target_transform.translation + inheritor.displacement;
        }
    });
}

pub struct StatusBarChasePlugin;

impl Plugin for StatusBarChasePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(
            CoreStage::PostUpdate,
            inherit_translation
            .after(bevy::transform::TransformSystem::ParentUpdate)
            .after(bevy::transform::TransformSystem::TransformPropagate)
        );
    }
}