pub mod menu_button_action;

use crate::common::game_state::{GameState, MenuState};
use crate::common::settings::{PortNameInput, ServiceNameInput, Settings};
use crate::menu::menu_button_action::{MenuButtonActionState, MenuButtonActions};
use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::ui::{FocusPolicy, Node};
use bevy_kira_audio::{AudioControl, AudioSource};
use bevy_persistent::prelude::*;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputPlugin, TextInputSettings,
    TextInputSystem, TextInputTextColor, TextInputTextFont, TextInputValue,
};
use std::path::Path;

pub struct MenuPlugin;
#[derive(Component)]
pub struct QuitButton;
const COLOR_BUTTON: Color = Color::srgb(1.0, 0.5, 0.0);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::WHITE;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.enable_state_scoped_entities::<MenuState>();
        app.add_systems(
            OnEnter(GameState::Menu),
            (setup, play_menu_music.after(setup)),
        );
        // 这里只有到了游戏内才更改！
        app.add_systems(OnEnter(GameState::Game), (stop_menu_music, disable));
        app.add_systems(
            Update,
            (toggle_quit, button_actions).run_if(in_state(GameState::Menu)),
        );
        app.add_systems(OnEnter(MenuState::Quit), quit_menu);
        app.add_systems(Update, quit_system.run_if(in_state(MenuState::Quit)));
        app.add_systems(OnEnter(MenuState::Settings), setup_setting);
        //FIXME: 这里可以提到全局
        app.add_systems(Update, (focus, listen_ime_events).before(TextInputSystem));
    }
}

#[derive(Resource)]
struct MenuMusicHandle(Handle<AudioSource>);

fn play_menu_music(audio: Res<bevy_kira_audio::Audio>, music_handle: Res<MenuMusicHandle>) {
    // 播放背景音乐并循环
    if !audio.is_playing_sound() {
        audio.play(music_handle.0.clone()).looped();
        info!("Menu music started.");
    }
}

fn stop_menu_music(audio: Res<bevy_kira_audio::Audio>) {
    // 停止所有音乐
    audio.stop();
    info!("Menu music stopped.");
}

fn disable(mut next_state: ResMut<NextState<MenuState>>) {
    next_state.set(MenuState::Disable);
}

// 初始化页面和ui
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    //setting startup
    let config_dir = dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("9tcg-2d"))
        .unwrap_or(Path::new("session").join("settings"))
        .join("myConfig");
    commands.insert_resource(
        Persistent::<Settings>::builder()
            .name("key bindings")
            .format(StorageFormat::Toml)
            .path(config_dir.join("key-bindings.toml"))
            .default(Settings {
                service: "127.0.0.1".to_string(),
                port: "28892".to_string(),
                ext_dir: "./ext".to_string(),
            })
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize key bindings"),
    );

    next_state.set(MenuState::Main);
    // 加载背景音乐
    let menu_music = asset_server.load("main/bgm.mp3"); // 替换为实际音乐文件路径
    commands.insert_resource(MenuMusicHandle(menu_music));
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
                    spawn_button(
                        left_plane,
                        font.clone(),
                        COLOR_BUTTON,
                        MenuButtonActions(MenuButtonActionState::Shop),
                        "Shop",
                    );
                    spawn_button(
                        left_plane,
                        font.clone(),
                        COLOR_BUTTON,
                        MenuButtonActions(MenuButtonActionState::Desk),
                        "卡组",
                    );
                    spawn_button(
                        left_plane,
                        font.clone(),
                        COLOR_BUTTON,
                        MenuButtonActions(MenuButtonActionState::Duel),
                        "决斗",
                    );
                    spawn_button(
                        left_plane,
                        font.clone(),
                        COLOR_BUTTON,
                        MenuButtonActions(MenuButtonActionState::Setting),
                        "设置",
                    );
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
    action: MenuButtonActions,
    text: &str,
) {
    builder
        .spawn((
            Button,
            action,
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

fn button_actions(
    interaction_query: Query<
        (&Interaction, &MenuButtonActions),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<Persistent<Settings>>,
    (service_query, port_query): (
        Query<&TextInputValue, With<ServiceNameInput>>,
        Query<&TextInputValue, With<PortNameInput>>,
    ),
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action.0.clone() {
                MenuButtonActionState::Shop => {
                    info!("Click On Shop!")
                }
                MenuButtonActionState::Desk => {
                    info!("Click On Desk!");
                    next_menu_state.set(MenuState::Disable);
                    next_game_state.set(GameState::Desk);
                }
                MenuButtonActionState::Setting => {
                    info!("Click On Setting!");
                    next_menu_state.set(MenuState::Settings);
                }
                MenuButtonActionState::Duel => {
                    info!("Click On Duel!");
                    next_game_state.set(GameState::Game);
                }
                MenuButtonActionState::Save => {
                    info!("Click On Save!");
                    settings
                        .update(|settings| {
                            if !service_query.is_empty() {
                                let service = service_query.get_single().unwrap();
                                settings.service = service.0.clone();
                            }
                            if !port_query.is_empty() {
                                let port = port_query.get_single().unwrap();
                                settings.port = port.0.clone();
                            }
                        })
                        .expect("failed to update settings");
                    next_menu_state.set(MenuState::Main);
                }
                MenuButtonActionState::Cancel => {
                    info!("Click On Cancel!");
                    next_menu_state.set(MenuState::Main);
                }
            }
        }
    }
}

// setting
fn setup_setting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<Settings>>,
) {
    let font = asset_server.load("fonts/wqy-microhei.ttc");

    commands
        .spawn((
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(50.0),
                margin: UiRect::all(Val::Auto),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor(Color::WHITE),
            StateScoped(MenuState::Settings),
            BackgroundColor(COLOR_BUTTON),
        ))
        .with_children(|parent| {
            spawn_label_input(
                parent,
                "服务器".to_string(),
                settings.service.to_string(),
                font.clone(),
                ServiceNameInput,
            );
            spawn_label_input(
                parent,
                "端口号".to_string(),
                settings.port.to_string(),
                font.clone(),
                PortNameInput,
            );
            // 按钮容器
            parent.spawn(
                (Node {
                    width: Val::Percent(100.),
                    height: Val::Px(50.),
                    margin: UiRect {
                        top: Val::Px(20.),
                        bottom: Val::Px(20.),
                        ..default()
                    },
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                }),
            );
        })
        .with_children(|button_plane| {
            // 保存和取消按钮
            button_plane
                .spawn((
                    MenuButtonActions(MenuButtonActionState::Save),
                    Button,
                    BackgroundColor(Color::srgb(0.15, 0.65, 0.15)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("保存"),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            button_plane
                .spawn((
                    MenuButtonActions(MenuButtonActionState::Cancel),
                    Button,
                    BackgroundColor(Color::srgb(0.65, 0.15, 0.15)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("取消"),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn spawn_label_input(
    builder: &mut ChildBuilder,
    label: String,
    value: String,
    font: Handle<Font>,
    component: impl Component,
) {
    builder
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(30.),
                margin: UiRect {
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            Interaction::None,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor::BLACK,
            ));
            parent.spawn((
                component,
                Node {
                    width: Val::Px(200.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(BORDER_COLOR_ACTIVE),
                BackgroundColor(BACKGROUND_COLOR),
                TextInput,
                TextInputTextFont(TextFont {
                    font_size: 20.,
                    ..default()
                }),
                TextInputSettings {
                    retain_on_submit: true,
                    ..default()
                },
                TextInputInactive(true),
                FocusPolicy::Block,
                TextInputValue(value),
                TextInputTextColor(TextColor(Color::BLACK)),
            ));
        });
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut window: Single<&mut Window>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            window.ime_position = window.cursor_position().unwrap();
            window.ime_enabled = true;
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

fn listen_ime_events(
    mut events: EventReader<Ime>,
    mut edit_text: Query<(&mut TextInputValue, &mut TextInputInactive)>,
) {
    for event in events.read() {
        match event {
            Ime::Commit { value, .. } => {
                debug!("Commit {:?}", value);
                for (mut text, inactive) in edit_text.iter_mut() {
                    debug!("Commit {:?}", inactive.0);
                    if !inactive.0 {
                        text.0.push_str(value);
                    }
                }
            }
            _ => {
                debug!("Ime event: {:?}", event);
            }
        }
    }
}
