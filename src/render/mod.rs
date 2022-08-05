use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::VecHelper;
use crate::prelude::*;

const DEFAULT_BAR_Z_DEPTH: f32 = 950.0;

fn extract_status_bars(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    subject_query: Extract<Query<&GlobalTransform>>,
    status_bar_query: Extract<Query<(
        &StatBarColor,
        Option<&StatBarEmptyColor>,
        Option<&StatBarBorder>,
        &StatBarValue,
        &StatBarSize,
        &StatBarSubject,
        Option<&StatBarPosition>,
        Option<&StatBarZDepth>,
        Option<&StatBarAlignment>,
        Option<&StatBarOrientation>,
    )>>,
) {
    for (
        &StatBarColor(color), 
        empty_color_option, 
        border_option, 
        &StatBarValue(value), 
        size, 
        &StatBarSubject(subject), 
        position_option,
        z_option,
        _alignment_option,
        _orientation_option,
    ) in status_bar_query.iter() {
        let position = position_option.map(|&StatBarPosition(p)| p).unwrap_or(Vec2::ZERO);
        let z_depth = z_option.map(|&StatBarZDepth(z)| z).unwrap_or(DEFAULT_BAR_Z_DEPTH);
        if let Ok(translation) =
            subject_query
            .get(subject)
            .map(|subject_transform| 
                (subject_transform.translation().truncate() + position)
                .extend(z_depth)
            ) {
                
            let inner_size = size.full_length * Vec2::X + size.thickness * Vec2::Y;

            if let Some(border) = border_option {
                let border_size = 
                    inner_size
                    + border.thickness * Vec2::ONE;

                extracted_sprites.sprites.alloc().init(
                    ExtractedSprite {
                        entity: subject,
                        transform: GlobalTransform::from_translation(translation),
                        color: border.color,
                        rect: None,
                        custom_size: Some(border_size),
                        image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                        flip_x: false,
                        flip_y: false,
                        anchor: Default::default()
                    }
                );
            }
            
            if let Some(empty_color) = empty_color_option {
                extracted_sprites.sprites.alloc().init(
                    ExtractedSprite {
                        entity: subject,
                        transform: GlobalTransform::from_translation(translation),
                        color: empty_color.0,
                        rect: None,
                        custom_size: Some(inner_size),
                        image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                        flip_x: false,
                        flip_y: false,
                        anchor: Default::default()
                    }
                );
            }

            if 0.0 < value {
                let clamped_value = value.clamp(0.0, 1.0);
                let bar_size = 
                    clamped_value * inner_size.x * Vec2::X 
                    + inner_size.y * Vec2::Y;
                let bar_translation = 
                    0.5 * size.full_length * (value - 1.0) * Vec3::X
                    + translation;

                extracted_sprites.sprites.alloc().init(
                    ExtractedSprite {
                        entity: subject,
                        transform: GlobalTransform::from_translation(bar_translation),
                        color,
                        rect: None,
                        custom_size: Some(bar_size),
                        image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                        flip_x: false,
                        flip_y: false,
                        anchor: Default::default()
                    }
                );
            }

        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum StatBarSystem {
    ExtractStatBars,
}

pub struct RenderStatBarsPlugin;

impl Plugin for RenderStatBarsPlugin {
    fn build(&self, app: &mut App) {

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
            .add_system_to_stage(
                RenderStage::Extract,
                extract_status_bars
                .label(StatBarSystem::ExtractStatBars)
                .after(SpriteSystem::ExtractSprites)
            );
        }
    }
}