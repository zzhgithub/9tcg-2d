use crate::common::game_state::{GameState, MenuState};
use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::ui::Node;

pub struct MenuPlugin;
#[derive(Component)]
pub struct QuitButton;
const COLOR_BUTTON: Color = Color::srgb(1.0, 0.5, 0.0);
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.enable_state_scoped_entities::<MenuState>();
        app.add_systems(OnEnter(GameState::Menu), setup);
        app.add_systems(Update, toggle_quit.run_if(in_state(GameState::Menu)));
        app.add_systems(
            Update,
            (quit_system, quit_menu).run_if(in_state(MenuState::Quit)),
        );
    }
}

// 初始化页面和ui
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 加载背景音乐
    // todo
    // 背景图片
    let image = asset_server.load("main/bg.png");
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    commands
        .spawn((
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            ImageNode::new(image),
            StateScoped(GameState::Menu),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(33.3),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center, // 垂直居中
                    align_items: AlignItems::FlexStart,      // 水平从上到下排列
                    flex_direction: FlexDirection::Column,   // 垂直排列子元素
                    padding: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                })
                .with_children(|left_plane| {
                    // 按钮的位置
                    spawn_button(left_plane, font.clone(), COLOR_BUTTON, "Shop");
                    spawn_button(left_plane, font.clone(), COLOR_BUTTON, "卡组");
                    spawn_button(left_plane, font.clone(), COLOR_BUTTON, "决斗");
                    spawn_button(left_plane, font.clone(), COLOR_BUTTON, "设置");
                });
            parent.spawn(Node {
                width: Val::Percent(33.3),
                height: Val::Percent(100.),
                ..Default::default()
            });
            parent.spawn(Node {
                width: Val::Percent(33.3),
                height: Val::Percent(100.),
                ..Default::default()
            });
        });
}

fn spawn_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    background_color: Color,
    text: &str,
) {
    builder
        .spawn((
            Button,
            Node {
                width: Val::Percent(25.),
                height: Val::Px(50.),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center, // 按钮文字居中
                align_items: AlignItems::Center,         // 垂直居中
                border: UiRect::all(Val::Px(2.0)),       // 白色边框
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(background_color),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn toggle_quit(
    state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<MenuState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            MenuState::Quit => {
                next_state.set(MenuState::Main);
            }
            _ => {
                next_state.set(MenuState::Quit);
            }
        }
    }
}

fn quit_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 一个退出按钮可以退出游戏
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            StateScoped(MenuState::Quit),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    QuitButton,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(1.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0., 0.)),
                    BorderColor(Color::BLACK),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("退出"),
                        TextFont {
                            font: asset_server.load("fonts/wqy-microhei.ttc"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

fn quit_system(
    interaction_query: Query<(&Interaction, &QuitButton), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, _quit_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            app_exit_events.send(AppExit::Success);
        }
    }
}
