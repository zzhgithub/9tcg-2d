use bevy::prelude::Component;

#[derive(Clone, Debug)]
pub enum MenuButtonActionState {
    Shop,
    Desk,
    Setting,
    Duel,
}

#[derive(Component, Clone, Debug)]
pub struct MenuButtonActions(pub(crate) MenuButtonActionState);
