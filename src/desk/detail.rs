use crate::common::desks_datas::{DeskData, DesksDataList};
use crate::common::game_state::DeskState;
use crate::common::test_data::{ALL_CARD, ALL_CARD_ONCE};
use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::desk::layout_back_button_and_content;
use crate::desk::scroll_list::{scroll_list, table_t};
use crate::utils::preview_plugins::ImagePreview;
use crate::utils::{BACKGROUND_COLOR, BORDER_COLOR_ACTIVE};
use bevy::prelude::*;
use bevy::text::cosmic_text::Motion::Next;
use bevy::ui::FocusPolicy;
use bevy_persistent::Persistent;
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};

#[derive(Debug, Resource)]
pub struct DeskSelect(pub Option<usize>);

#[derive(Resource)]
pub struct CurrentDeskData(pub Option<DeskData>);

#[derive(Component)]
pub struct DeskPlane;

pub fn open_desk_detail(
    mut commands: Commands,
    desk_list: Res<Persistent<DesksDataList>>,
    desk_select: Res<DeskSelect>,
    asset_server: Res<AssetServer>,
) {
    // 打开详情
    let list_array: &[DeskData] = &desk_list.list;
    let mut name = "".to_string();
    let mut current: Option<DeskData> = None;
    //加载当前数组
    if let Some(selected) = desk_select.0 {
        current = Some(list_array[selected].clone());
        commands.insert_resource(CurrentDeskData(Some(list_array[selected].clone())));
        name = list_array[selected].name.clone();
    } else {
        commands.insert_resource(CurrentDeskData(Some(DeskData {
            name: name.clone(),
            cards: vec!["S001-A-001".to_string()],
        })));
    }
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    layout_back_button_and_content(
        commands,
        font.clone(),
        DeskState::Detail,
        Box::from([
            ("返回", DeskButtonActions(DeskButtonActionState::BackToDesk)),
            ("Save", DeskButtonActions(DeskButtonActionState::Save)),
            ("Use!", DeskButtonActions(DeskButtonActionState::Use)),
        ]),
        |plane| {
            plane
                .spawn((Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },))
                .with_children(|parent| {
                    // 上
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Auto,
                                flex_direction: FlexDirection::Row, // 水平排列子节点
                                align_items: AlignItems::FlexStart, // 垂直对齐方式
                                justify_content: JustifyContent::FlexStart, // 左对齐
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.8, 0.6)),
                        ))
                        .with_children(|header| {
                            // 这里是一个输入框
                            header.spawn((
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
                                TextInputValue(name.clone()),
                                DeskNameInput,
                                TextInputTextColor(TextColor(Color::BLACK)),
                            ));
                        });
                    // 下
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                margin: UiRect {
                                    top: Val::Px(10.),
                                    ..default()
                                },
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ))
                        .with_children(|parent| {
                            // 左边卡组
                            parent
                                .spawn((Node {
                                    width: Val::Percent(70.0),
                                    height: Val::Percent(90.0),
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },))
                                .with_children(|content_plane| {
                                    content_plane
                                        .spawn((
                                            Node {
                                                display: Display::Flex,
                                                width: Val::Percent(96.),
                                                height: Val::Percent(93.),
                                                flex_direction: FlexDirection::Column,
                                                margin: UiRect::all(Val::Px(5.)),
                                                overflow: Overflow::scroll_y(),
                                                ..default()
                                            },
                                            BackgroundColor(Color::NONE),
                                            DeskPlane,
                                        ))
                                        .with_children(|content_plane| {
                                            let cards = if let Some(current) = current {
                                                &current.cards.clone()
                                            } else {
                                                // 第一次创建 默认卡组第一张卡
                                                &vec!["S001-A-001".to_string()]
                                            };
                                            spawn_desks(content_plane, &cards, &asset_server);
                                        });
                                });
                            // 右边搜索
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(90.0),
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(Color::srgb(1.0, 0.0, 1.0)),
                                ))
                                .with_children(|content_plane| {
                                    // ToDo 这里是搜索内容
                                    scroll_list(
                                        content_plane,
                                        &ALL_CARD_ONCE,
                                        3,
                                        |row, t, _index| {
                                            let image =
                                                asset_server.load(format!("cards/min/{}.png", t));
                                            row.spawn((
                                                ImageNode {
                                                    image: image.clone(),
                                                    ..default()
                                                },
                                                Node {
                                                    width: Val::Percent(100.0 / 3.0),
                                                    padding: UiRect::all(Val::Px(2.0)),
                                                    ..default()
                                                },
                                                BorderRadius::all(Val::Px(2.0)),
                                                Button,
                                                Interaction::None,
                                                ImagePreview(format!("{}", t)),
                                                CardCode(String::from(*t)),
                                            ))
                                            .observe(on_right_click)
                                            .insert(
                                                PickingBehavior {
                                                    should_block_lower: false,
                                                    ..default()
                                                },
                                            );
                                        },
                                    );
                                });
                        });
                });
        },
    );
}

pub fn spawn_desks<T: std::fmt::Display>(
    mut content_plane: &mut ChildBuilder,
    cards: &[T],
    asset_server: &Res<AssetServer>,
) {
    table_t(content_plane, cards, 10, |row, t, index| {
        let image = asset_server.load(format!("cards/min/{}.png", t));
        row.spawn((
            ImageNode {
                image: image.clone(),
                ..default()
            },
            Node {
                width: Val::Percent(100.0 / 10.0),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(2.0)),
            Button,
            Interaction::None,
            ImagePreview(format!("{}", t)),
            CardIndex(index),
        ))
        .observe(on_right_click_remove)
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        });
    });
}

#[derive(Debug, Component)]
pub struct CardCode(pub String);
#[derive(Debug, Component)]
pub struct CardIndex(pub usize);
#[derive(Debug, Component)]
pub struct DeskNameInput;

pub fn on_right_click_remove(
    click: Trigger<Pointer<Click>>,
    mut query: Query<&mut CardIndex>,
    mut current_data: ResMut<CurrentDeskData>,
) {
    if let Ok(mut card_index) = query.get_mut(click.entity()) {
        info!("Clicked on cardIndex {}", card_index.0);
        debug!("Event {:?}", click.event);
        if click.button == PointerButton::Secondary {
            // 鼠标左键
            if let Some(mut desk) = current_data.0.clone() {
                desk.cards.remove(card_index.0);
                current_data.0 = Some(desk);
            }
        }
    }
}

pub fn on_right_click(
    click: Trigger<Pointer<Click>>,
    mut query: Query<&mut CardCode>,
    mut current_data: ResMut<CurrentDeskData>,
) {
    if let Ok(mut card_code) = query.get_mut(click.entity()) {
        info!("Clicked on cardcode {}", card_code.0);
        debug!("Event {:?}", click.event);
        if click.button == PointerButton::Secondary {
            // 鼠标左键
            if let Some(mut desk) = current_data.0.clone() {
                desk.cards.push(card_code.0.clone());
                current_data.0 = Some(desk);
            }
        }
    }
}

pub fn on_data_changed(
    current_data: Res<CurrentDeskData>,
    query: Single<(Entity, &Children), With<DeskPlane>>,
    child_query: Query<Entity, With<Parent>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if current_data.is_changed() {
        info!("Changed");
        let (entity, children) = query.into_inner();
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                commands.entity(child).despawn_recursive();
            }
        }
        // 重新刷新数据
        if let Some(mut parent) = commands.get_entity(entity) {
            if let Some(mut desk) = current_data.0.clone() {
                parent.with_children(|parent| {
                    spawn_desks(parent, &desk.cards, &asset_server);
                });
            }
        }
    }
}
