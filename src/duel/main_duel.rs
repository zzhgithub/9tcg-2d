use crate::common::game_state::{DuelState, GameState};
use crate::utils::{BACKGROUND_COLOR, BORDER_COLOR_ACTIVE, COLOR_BUTTON};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};

#[derive(Component)]
pub struct LocalPortInput;
#[derive(Component)]
pub struct PlayerPortInput;
#[derive(Component)]
pub struct MyNumInput;

#[derive(Component)]
pub struct DuelMainAction(pub DuelMainActionType);

pub enum DuelMainActionType {
    Start,
    Back,
}

pub fn setup_duel(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 创建一个返回的按钮
    // 游戏的场地画面
    // 基本UI
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            StateScoped(DuelState::Main),
        ))
        .with_children(|parent| {
            create_label_and_input(parent, font.clone(), "本地端口", LocalPortInput);
            create_label_and_input(parent, font.clone(), "对方端口", PlayerPortInput);
            create_label_and_input(parent, font.clone(), "本机编号", MyNumInput);

            spawn_button(
                parent,
                font.clone(),
                COLOR_BUTTON,
                "开始游戏",
                DuelMainAction(DuelMainActionType::Start),
            );
            spawn_button(
                parent,
                font.clone(),
                COLOR_BUTTON,
                "返回",
                DuelMainAction(DuelMainActionType::Back),
            );
        });
}

fn spawn_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    background_color: Color,
    text: &str,
    action_component: DuelMainAction,
) {
    builder
        .spawn((
            Button,
            action_component,
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

fn create_label_and_input<T>(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    label_text: &str,
    input_component: T,
) where
    T: Component,
{
    parent
        .spawn(
            (Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            }),
        )
        .with_children(|parent| {
            parent.spawn((Text::new(label_text), TextFont {
                font: font.clone(),
                font_size: 30.0,
                font_smoothing: Default::default(),
            }));
            //
            parent.spawn((
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
                TextInputValue("".to_string()),
                TextInputTextColor(TextColor(Color::BLACK)),
                input_component,
            ));
        });
}

pub fn handle_main_button(
    interaction_query: Query<(&Interaction, &DuelMainAction), (Changed<Interaction>, With<Button>)>,
    local_port_input: Single<&TextInputValue, With<LocalPortInput>>,
    player_port_input: Single<&TextInputValue, With<PlayerPortInput>>,
    my_num_input: Single<&TextInputValue, With<MyNumInput>>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_duel_state: ResMut<NextState<DuelState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action.0 {
                DuelMainActionType::Start => {
                    // 尝试建立连接
                    info!("尝试建立连接");
                    let x = local_port_input.0.clone();
                    let y = player_port_input.0.clone();
                    let z = my_num_input.0.clone();
                    // todo
                    info!("local port:{}, player port:{}, my_num_input:{}", x, y, z,);
                }
                DuelMainActionType::Back => {
                    next_duel_state.set(DuelState::Disable);
                    next_game_state.set(GameState::Menu);
                }
            }
        }
    }
}
