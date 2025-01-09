// 卡组列表

use crate::common::desks_datas::{DeskData, DesksDataList};
use crate::common::game_state::DeskState;
use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::desk::detail::DeskSelect;
use crate::desk::layout_back_button_and_content;
use crate::desk::scroll_list::scroll_list;
use bevy::prelude::*;
use bevy_persistent::{Persistent, StorageFormat};
use std::path::Path;

#[derive(Component)]
pub struct DeskIndex(pub usize);

// 初始化Desks
pub fn setup_desks(mut commands: Commands) {
    let config_dir = dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("9tcg-2d"))
        .unwrap_or(Path::new("session").join("settings"))
        .join("myConfig");
    commands.insert_resource(
        Persistent::<DesksDataList>::builder()
            .name("Desk List")
            .format(StorageFormat::Toml)
            .path(config_dir.join("desks.toml"))
            .default(DesksDataList {
                list: vec![DeskData {
                    name: "默认卡组".to_string(),
                    cards: vec!["S001-A-004".to_string()],
                }],
                used: 0,
            })
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize Desks list"),
    );
}

pub fn list_desks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    desk_list: Res<Persistent<DesksDataList>>,
) {
    let list_array: &[DeskData] = &desk_list.list;
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    layout_back_button_and_content(
        commands,
        font.clone(),
        DeskState::Desks,
        Box::from([
            ("返回", DeskButtonActions(DeskButtonActionState::BackToMenu)),
            ("新建", DeskButtonActions(DeskButtonActionState::NewDesk)),
        ]),
        |plane| {
            plane
                .spawn(
                    (Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    }),
                )
                .with_children(|parent| {
                    scroll_list(parent, list_array, 6, |row, t, index| {
                        let image = asset_server.load("desk/mu.png");
                        row.spawn((
                            ImageNode {
                                image: image.clone(),
                                ..default()
                            },
                            Node {
                                width: Val::Px(300.),
                                height: Val::Px(300.),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(5.0)),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            Button,
                            DeskIndex(index),
                            Outline {
                                width: Val::Px(5.),
                                offset: Val::Px(5.),
                                color: if index == desk_list.used {
                                    Color::srgb(0.0, 1.0, 0.0)
                                } else {
                                    Color::NONE
                                },
                            },
                            Interaction::None,
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Node { ..default() },
                                Text::new(t.name.clone()),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor::WHITE,
                            ));
                        });
                    });
                });
        },
    );
}

pub fn handel_click_desk(
    mut query: Query<(&Interaction, &mut DeskIndex), With<Button>>,
    mut next_state: ResMut<NextState<DeskState>>,
    mut desk_select: ResMut<DeskSelect>,
) {
    query.iter_mut().for_each(|(interaction, mut index)| {
        if (*interaction == Interaction::Pressed) {
            desk_select.0 = Some(index.0);
            next_state.set(DeskState::Detail);
        }
    })
}
