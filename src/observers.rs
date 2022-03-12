use std::marker::PhantomData;
use bevy::prelude::*;

#[derive(Component)]
pub struct Follow {
    pub target: Entity,
    pub displacement: Vec2,
}

#[derive(Component)]
pub struct Observe {
    pub target: Entity
}

#[derive(Default)]
#[derive(Component)]
pub struct ObserveComponent<C, F> 
where
    F: FnMut(&C) -> f32,
{
    pub target: Option<Entity>,
    pub component: PhantomData<C>,
    pub function: F
}

#[derive(Default)]
#[derive(Component)]
pub struct ObserveResource<R, F> 
where
    F: FnMut(&R) -> f32,
{
    pub resource: PhantomData<R>,
    pub function: F
}


pub struct ObserverPlugin;

impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}