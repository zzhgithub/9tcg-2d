use bevy::prelude::Component;

#[derive(Clone, Debug)]
pub enum DeskButtonActionState {
    BackToMenu,
    BackToList,
    NewDesk,
}

#[derive(Component, Clone, Debug)]
pub struct DeskButtonActions(pub DeskButtonActionState);
