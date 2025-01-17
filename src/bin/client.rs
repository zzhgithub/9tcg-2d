use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowPlugin, WindowTheme};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_simple_text_input::TextInputPlugin;
use tcg_2d::common::game_state::GameState;
use tcg_2d::desk::DeskPlugins;
use tcg_2d::duel::DuelPlugin;
use tcg_2d::menu::MenuPlugin;
use tcg_2d::splash::SplashPlugin;
use tcg_2d::utils::preview_plugins::PreviewPlugins;

fn main() {
    let mut app = App::new();
    // 全屏显示
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "9TCG".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                // resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }),
        AudioPlugin,
    ));
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
    // 加载不同的插件
    app.add_plugins((
        TextInputPlugin,
        PreviewPlugins,
        SplashPlugin,
        MenuPlugin,
        DeskPlugins,
        DuelPlugin,
        // WorldInspectorPlugin::new(),
    ));

    app.run();
}
