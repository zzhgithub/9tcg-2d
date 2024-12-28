use bevy::prelude::States;

#[derive(PartialEq, Clone, Eq, Copy, Default, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Desk,
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
