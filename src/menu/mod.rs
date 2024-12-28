use crate::common::game_state::{GameState, MenuState};
use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::ui::Node;

pub struct MenuPlugin;
#[derive(Component)]
pub struct QuitButton;

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
    let image = asset_server.load("main/bg.png");
    commands.spawn((
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ImageNode::new(image),
        StateScoped(GameState::Menu),
    ));
    // todo 添加排版和按钮
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

fn quit_menu(mut commands: Commands) {
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
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextFont {
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
