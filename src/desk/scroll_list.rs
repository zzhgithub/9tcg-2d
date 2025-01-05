use bevy::color::Color;
use bevy::hierarchy::ChildBuilder;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::focus::HoverMap;
use bevy::prelude::*;
use bevy::prelude::{BackgroundColor, Display, FlexDirection, Node, Overflow, Val, default};

pub fn scroll_list<T, F>(
    parent: &mut ChildBuilder,
    list: &[T],
    items_per_row: usize,
    mut callback: F,
) where
    F: FnMut(&mut ChildBuilder, &T, usize),
{
    parent
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
        ))
        .with_children(|parent| {
            table_t(parent, list, items_per_row, &mut callback);
        });
}

pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 5.0, mouse_wheel_event.y * 5.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

pub fn table_t<T, F>(parent: &mut ChildBuilder, list: &[T], items_per_row: usize, mut callback: F)
where
    F: FnMut(&mut ChildBuilder, &T, usize),
{
    let last_item = list.len() % items_per_row;
    let row_num = list.len() / items_per_row + if (last_item != 0) { 1 } else { 0 };

    for oi in 0..row_num {
        parent
            .spawn(
                (Node {
                    flex_direction: FlexDirection::Row,
                    ..default()
                }),
            )
            .insert(PickingBehavior {
                should_block_lower: false,
                ..default()
            })
            .with_children(|row| {
                let col_num = if (oi == row_num - 1 && last_item != 0) {
                    last_item
                } else {
                    items_per_row
                };
                // 便利每行
                for i in 0..col_num {
                    let index = oi * items_per_row + i;
                    callback(row, &list[index], index);
                }
            });
    }
}
