// use bevy::ecs::system::EntityCommands;
// use bevy::prelude::*;
// use chase::InheritTranslation;
// use crate::*;

// pub fn spawn_status_bar(
//     transform: Transform,
//     bar: StatusBar,
//    commands: &mut EntityCommands,
// ) {
//     let value = bar.value.clamp(0.0, 1.0);
//     let inner_size = vec2(bar.full_length, bar.thickness);
//     let bar_translation = 0.5 * bar.full_length * (value - 1.0) * Vec3::X;
//     commands
//     .with_children(|builder| {
//         builder.spawn_bundle(SpriteBsundle {
//             sprite: Sprite { 
//                 color: bar.full_color,
//                 custom_size: Some(inner_size.y * Vec2::Y + value * bar.full_length * Vec2::X),
//                 ..Default::default()
//             },
//             transform: transform.with_translation(transform.translation + bar_translation + 0.1 * Vec3::Z),
//             ..Default::default()
//         });
//         builder.spawn_bundle(SpriteBundle {
//             sprite: Sprite { 
//                 color: bar.empty_color,
//                 custom_size: Some(inner_size),
//                 ..Default::default()
//             },
//             transform: transform.with_translation(transform.translation + 0.1 * Vec3::Z),
//             ..Default::default()
//         });
        
        
//         if let Some(border) = bar.border {
//             let outer_size = inner_size + 2.0 * Vec2::ONE * border.thickness;
//             builder.spawn_bundle(SpriteBundle {
//                 sprite: Sprite { 
//                     color: border.color,
//                     custom_size: Some(outer_size),
//                     ..Default::default()
//                 },
//                 transform,
//                 ..Default::default()
//             });
//         }
//     });

// }

// fn spawn_status_bar_sprites(
//     mut commands: Commands,
//     mut status_bar: Query<(Entity, &StatusBar, &InheritTranslation), Without<StatusBarSprites>>,
// ) {
//     status_bar.for_each_mut(|(bar_entity, bar, &inheritance)| {
//         println!("spawning sprites etc");
//         let value = bar.value.clamp(0.0, 1.0);
//         let inner_size = vec2(bar.full_length, bar.thickness);
//         let bar_translation = 0.5 * bar.full_length * (value - 1.0) * Vec3::X;
//         let sprites = StatusBarSprites {
//             bar_sprite:
//                 commands.spawn_bundle(SpriteBundle {
//                     sprite: Sprite { 
//                         color: bar.full_color,
//                         custom_size: Some(inner_size.y * Vec2::Y + value * bar.full_length * Vec2::X),
//                         ..Default::default()
//                     },
//                     transform: Transform::from_translation(bar_translation + 0.2 * Vec3::Z),
//                     ..Default::default()
//                 })
//                 .insert(inheritance)
//                 .id(),
//             empty_sprite:
//                 commands.spawn_bundle(SpriteBundle {
//                     sprite: Sprite { 
//                         color: bar.empty_color,
//                         custom_size: Some(inner_size),
//                         ..Default::default()
//                     },
//                     transform: Transform::from_translation(0.1 * Vec3::Z),
//                     ..Default::default()
//                 })
//                 .insert(inheritance)
//                 .id(),
//             border_sprite:
//                 if let Some(ref border) = bar.border {
//                     let outer_size = inner_size + 2.0 * Vec2::ONE * border.thickness;
//                     commands.spawn_bundle(SpriteBundle {
//                         sprite: Sprite { 
//                             color: border.color,
//                             custom_size: Some(outer_size),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     })
//                     .insert(inheritance)
//                     .id().into()
//                 } else {
//                     None
//                 },
//             };
//         commands.entity(bar_entity).add_child(sprites.bar_sprite).add_child(sprites.empty_sprite);
//         if let Some(border_sprite) = sprites.border_sprite {
//             commands.entity(bar_entity).add_child(border_sprite);
//         }
//         commands.entity(bar_entity).insert(sprites);
//     });
// }

// pub struct SpawnBarSpritesPlugin;

// impl Plugin for SpawnBarSpritesPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         .add_system_to_stage(
//             CoreStage::PreUpdate,
//             spawn_status_bar_sprites
//         );
//     }
// }