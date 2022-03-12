use bevy::prelude::*;
use crate::prelude::*;

pub(crate) fn despawn_if_subject_not_found(
    mut commands: Commands,
    subjects: Query<Entity>,
    bars: Query<(Entity, &StatBarSubject)>,
) {
    bars.for_each(|(bar, &StatBarSubject(subject))| {
        if subjects.get(subject).is_err() {
            commands.entity(bar).despawn();
        }
    });
}