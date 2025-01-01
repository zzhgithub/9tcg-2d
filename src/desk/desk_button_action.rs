use bevy::prelude::Component;

#[derive(Clone, Debug)]
pub enum DeskButtonActionState {
    BackToMenu,
    BackToList,
}

#[derive(Component, Clone, Debug)]
pub struct DeskButtonActions(pub DeskButtonActionState);
