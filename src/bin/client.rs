use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowPlugin};
use tcg_2d::common::game_state::GameState;
use tcg_2d::splash::SplashPlugin;

fn main() {
    let mut app = App::new();
    // 全屏显示
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "9TCG".to_string(),
            mode: WindowMode::Fullscreen(MonitorSelection::Primary),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
    // 加载不同的插件
    app.add_plugins(SplashPlugin);

    app.run();
}
