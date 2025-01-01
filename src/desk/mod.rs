mod desk_button_action;
mod list;

use crate::common::game_state::{DeskState, GameState, MenuState};
use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::menu::menu_button_action::MenuButtonActions;
use bevy::prelude::*;
use bevy::utils::info;

pub struct DeskPlugins;

impl Plugin for DeskPlugins {
    fn build(&self, app: &mut App) {
        app.init_state::<DeskState>();
        app.enable_state_scoped_entities::<DeskState>();
        app.add_systems(OnEnter(GameState::Desk), setup);
        app.add_systems(Update, button_actions.run_if(in_state(GameState::Desk)));

        app.add_systems(OnEnter(DeskState::List), list::list_page);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<DeskState>>,
) {
    next_state.set(DeskState::List);
}

// 默认的布局页面

fn layout_back_button_and_content(
    mut commands: Commands,
    font: Handle<Font>,
    back_action: DeskButtonActions,
    spawn_content: impl FnOnce(&mut ChildBuilder),
) {
    commands
        .spawn((
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            StateScoped(GameState::Desk),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Baseline,
                        flex_direction: FlexDirection::ColumnReverse,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.8, 0.6)),
                ))
                .with_children(|header| {
                    header
                        .spawn((
                            back_action,
                            Button,
                            Node {
                                width: Val::Px(80.),
                                height: Val::Px(40.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.5, 0.0)),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("返回"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 20.,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });
                });
            // 内容区域
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        margin: UiRect {
                            top: Val::Px(10.),
                            ..default()
                        },
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|parent| {
                    spawn_content(parent);
                });
        });
}

fn button_actions(
    interaction_query: Query<
        (&Interaction, &DeskButtonActions),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_desk_state: ResMut<NextState<DeskState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action.0.clone() {
                DeskButtonActionState::BackToMenu => {
                    info!("Back to Menu Page");
                    next_desk_state.set(DeskState::Disable);
                    next_game_state.set(GameState::Menu);
                }
                DeskButtonActionState::BackToList => {
                    info("Back to Desk list");
                    next_desk_state.set(DeskState::List);
                }
            }
        }
    }
}
