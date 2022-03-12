use crate::prelude::*;

/// basic components 
/// required by all stat bars
pub struct StatBarRequiredBundle {
    pub color: StatBarColor,
    pub value: StatBarValue,
    pub size: StatBarSize,
    pub subject: StatBarSubject
}

pub struct StatBarBundle {
    pub color: StatBarColor,
    pub empty_color: StatBarEmptyColor,
    pub border: StatBarBorder,
    pub value: StatBarValue,
    pub size: StatBarSize,
    pub subject: StatBarSubject,
    pub position: StatBarPosition
}