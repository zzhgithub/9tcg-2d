use crate::VERSION;
use crate::common::game_state::GameState;
use bevy::app::App;
use bevy::log::info;
use bevy::prelude::{
    AlignItems, AssetServer, BuildChildren, Camera2d, ChildBuild, Commands, Deref, DerefMut,
    ImageNode, IntoSystemConfigs, JustifyText, Name, NextState, OnEnter, Plugin, PositionType, Res,
    ResMut, Resource, Single, StateScoped, Text, TextLayout, Time, Timer, TimerMode, Update,
    Window, in_state,
};
use bevy::text::TextFont;
use bevy::ui::{JustifyContent, Node, Val};
use bevy::utils::default;

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
            parent.spawn((
                Text::new("9TCG Ver:".to_owned() + VERSION),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
            ));
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
