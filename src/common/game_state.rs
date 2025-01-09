use bevy::prelude::States;

#[derive(PartialEq, Clone, Eq, Copy, Default, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Desk,
    Shop,
    Game,
}

#[derive(PartialEq, Clone, Eq, Copy, Default, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Disable,
    Main,
    Settings,
    Quit,
}

#[derive(PartialEq, Clone, Eq, Copy, Default, Debug, Hash, States)]
pub enum DeskState {
    #[default]
    Disable,
    List,
    Desks,
    Detail,
}

#[derive(PartialEq, Clone, Eq, Copy, Default, Debug, Hash, States)]
pub enum DuelState {
    #[default]
    Disable,
    Main,
    Connecting,
    Connected,
}
