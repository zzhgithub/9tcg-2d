use bevy::prelude::Component;

#[derive(Clone, Debug)]
pub enum DeskButtonActionState {
    BackToMenu,
    BackToList,
    BackToDesk,
    NewDesk,
    Save,
    Use,
}

#[derive(Component, Clone, Debug)]
pub struct DeskButtonActions(pub DeskButtonActionState);
