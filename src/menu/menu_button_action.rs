use bevy::prelude::Component;

#[derive(Clone, Debug)]
pub enum MenuButtonActionState {
    Shop,
    Desk,
    Setting,
    Duel,
    Save,
    Cancel,
}

#[derive(Component, Clone, Debug)]
pub struct MenuButtonActions(pub(crate) MenuButtonActionState);
