use bevy::math::vec2;
use bevy::prelude::*;
use bevy_status_bars_2d::prelude::*;
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct Speed(f32);


pub fn spawn_player(
    mut commands: Commands
) {
    let player = commands
    .spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(32.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .insert(Speed(250.0))
    .insert(PlayerCharacter)
    .id();

    commands
    .spawn_bundle((
        StatusBar {
            ..Default::default()
        },
        InheritTranslation {
            target: player,
            displacement: 40.0 * Vec3::Y
        },
    ));    
}

pub fn move_player(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<PlayerCharacter>>
) {
    query.for_each_mut(|(mut transform, player_speed)| {
        let mut m = Vec3::ZERO;
        if keyboard.pressed(KeyCode::A) {
            m -= Vec3::X
        } 
        if keyboard.pressed(KeyCode::D) {
            m += Vec3::X
        }
        if keyboard.pressed(KeyCode::S) {
            m -= Vec3::Y
        }
        if keyboard.pressed(KeyCode::W) {
            m += Vec3::Y
        }
        transform.translation += time.delta_seconds() * player_speed.0 * m.normalize_or_zero();
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(StatusBarsPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })   
    .add_startup_system(spawn_player)
    .add_system(move_player)
    
    .run();
}