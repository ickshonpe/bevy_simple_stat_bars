use crate::*;

#[derive(Component)]
pub struct StatusBarBorder {
    pub color: Color,
    pub thickness: f32,
}

impl Default for StatusBarBorder {
    fn default() -> Self {
        Self { 
            color: Color::DARK_GRAY,
            thickness: 2.0
        }
    } 
}

#[derive(Component)]
pub enum StatusBarOrientation {
    /// left = 0.0, right = 1.0
    Horizontal,
    /// bottom = 0.0, top = 1.0
    Vertical,
    /// left = 1.0, right = 0.0
    HorizontalReverse,
    /// bottom = 1.0, top = 0.0
    VerticalReverse,
    Angle { radians: f32 }
}

impl Default for StatusBarOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

#[derive(Component)]
pub struct StatusBarValue(pub f32);

impl Default for StatusBarValue {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
pub struct StatusBarColor(pub Color);

impl Default for StatusBarColor {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

#[derive(Component)]
pub struct StatusBarAlignment(pub Vec2);

impl Default for StatusBarAlignment {
    fn default() -> Self {
        Self(0.5 * Vec2::ONE)
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct StatusBarSize {
    pub full_length: f32,
    pub thickness: f32,
}

impl Default for StatusBarSize {
    fn default() -> Self {
        Self {
            full_length: 100.0,
            thickness: 10.0,
        }
    }
}


#[derive(Component)]
pub struct StatusBarZDepth(pub f32);

impl Default for StatusBarZDepth {
    fn default() -> Self {
        Self(950.0)
    }
}

#[derive(Component)]
pub struct StatusBarEmptyColor(pub Color);

impl Default for StatusBarEmptyColor {
    fn default() -> Self {
        Self(Color::BLACK)
    }
}

#[derive(Component)]
pub struct StatusBar {
    pub value: f32,
    pub z_depth: f32,
    pub thickness: f32,
    pub full_length: f32,
    pub full_color: Color,
    pub empty_color: Color,
    pub alignment: Vec2,
    pub border: Option<StatusBarBorder>,
    pub orientation: StatusBarOrientation,  
}

impl Default for StatusBar {
    fn default() -> Self {
        Self { 
            value: 0.8, 
            z_depth: 900.0, 
            thickness: 8.0, 
            full_length: 64.0, 
            full_color: Color::WHITE, 
            empty_color: Color::BLACK, 
            alignment: 0.5 * Vec2::ONE,
            border: Some(Default::default()), 
            orientation: Default::default() }
    }
}


#[derive(Component)]
pub struct StatusBarSprites {
    pub border_sprite: Option<Entity>,
    pub empty_sprite: Entity,
    pub bar_sprite: Entity,
}

#[derive(Component)]
pub struct StatusBarSubject(pub Entity);