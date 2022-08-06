use bevy::math::vec2;
use bevy::prelude::*;
use bevy_simple_stat_bars::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct PlayerCharacter;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Hp { current: i32, max: i32 }

#[derive(Component)]
struct Mp { current: i32, max: i32 }

#[derive(Component)]
struct StatBars {
    pub hp: Entity,
    pub mp: Entity,
}

fn spawn_player(
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
        .insert(Hp { current: 30, max: 30 })
        .insert(Mp { current: 12, max: 15 })
        .id();

    let hp_bar = commands
        .spawn_bundle((
            StatBarColor(Color::GREEN),
            StatBarEmptyColor(Color::BLACK),
            StatBarBorder { color: Color::DARK_GRAY, thickness: 3.0 },
            StatBarValue(1.0),
            StatBarSize { full_length: 50.0, thickness: 6.0 },
            StatBarSubject(player),
            StatBarPosition(40.0 * Vec2::Y)
        )).id();    

    let mp_bar = commands
        .spawn_bundle((
            StatBarColor(Color::PURPLE),
            StatBarEmptyColor(Color::BLACK),
            StatBarBorder { color: Color::DARK_GRAY, thickness: 3.0 },
            StatBarValue(12.0 / 15.0),
            StatBarSize { full_length: 50.0, thickness: 6.0 },
            StatBarSubject(player),
            StatBarPosition(50.0 * Vec2::Y)
        )).id(); 

    commands.entity(player)
        .insert(StatBars {
            hp: hp_bar,
            mp: mp_bar,
        });
}

fn move_player(
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

fn update_stats(
    mut cooldown: Local<f32>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut players: Query<(&mut Hp, &mut Mp), With<PlayerCharacter>>,
) {
    *cooldown -= time.delta_seconds();
    if 0.0 < *cooldown { 
        return; 
    } else {
        *cooldown = 0.1;
    }
    players.for_each_mut(|(mut hp, mut mp)| {
        if keyboard.pressed(KeyCode::Down) {
            hp.current = (hp.current - 1).clamp(0, hp.max);
        } 
        if keyboard.pressed(KeyCode::Up) {
            hp.current = (hp.current + 1).clamp(0, hp.max);
        }
        if keyboard.pressed(KeyCode::Left) {
            mp.current = (mp.current - 1).clamp(0, mp.max);
        }
        if keyboard.pressed(KeyCode::Right) {
            mp.current = (mp.current + 1).clamp(0, mp.max);
        }
    });
}

fn update_bars(
    mut stats: Query<(&mut Hp, &mut Mp, &StatBars)>,
    mut stat_bars: Query<&mut StatBarValue>,
) {
    stats.for_each_mut(|(hp, mp, bars)| {
        if let Ok(mut hp_bar) = stat_bars.get_mut(bars.hp) {
            hp_bar.0 = hp.current as f32 / hp.max as f32;
        }

        if let Ok(mut mp_bar) = stat_bars.get_mut(bars.mp) {
            mp_bar.0 = mp.current as f32 / mp.max as f32;
        }
    });
}

fn death(
    mut commands: Commands,
    query: Query<(Entity, &Hp)>,
) {
    query.for_each(|(entity, hp)| {
        if hp.current <= 0 {
            commands.entity(entity).despawn();
        }
    });
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::NAVY))
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(Camera2dBundle::default()); })   
    .add_startup_system(spawn_player)
    .add_system(move_player)
    .add_system(death)
    .add_system(update_stats)
    .add_system(update_bars)
    .run();
}