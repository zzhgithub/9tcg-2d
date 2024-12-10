use crate::common::game_state::GameState;
use bevy::app::App;
use bevy::log::info;
use bevy::prelude::{
    AlignItems, AssetServer, BuildChildren, Camera2d, ChildBuild, Commands, Deref, DerefMut,
    ImageNode, IntoSystemConfigs, Name, NextState, OnEnter, Plugin, Res, ResMut, Resource,
    StateScoped, Time, Timer, TimerMode, Update, in_state,
};
use bevy::ui::{JustifyContent, Node, Val};

// 标题画面
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // setup 加载图片
        app.add_systems(OnEnter(GameState::Splash), setup);
        app.add_systems(Update, (countdown).run_if(in_state(GameState::Splash)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("splash/splash.jpeg");
    //UI camera
    commands.spawn((Name::from("Main_Camera"), Camera2d));
    // UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            StateScoped(GameState::Splash),
        ))
        .with_children(|parent| {
            parent.spawn(ImageNode::new(image));
        });

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        info!("Splash timer count down");
        game_state.set(GameState::Menu);
    }
}
