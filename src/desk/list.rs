use crate::common::test_data::ALL_CARD;
use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::desk::layout_back_button_and_content;
use crate::desk::scroll_list::scroll_list;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::ui::FocusPolicy;
use bevy::ui::widget::ImageNodeSize;
use log::info;

pub fn list_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/wqy-microhei.ttc");

    layout_back_button_and_content(
        commands,
        font,
        DeskButtonActions(DeskButtonActionState::BackToMenu),
        |plane| {
            //FIXME: 这里是测试使用的代码
            info!("Plane spawn");
            scroll_list(plane, &ALL_CARD, 10, |row, &t| {
                row.spawn((
                    ImageNode {
                        image: asset_server.load(format!("cards/{}.png", t)),
                        ..default()
                    },
                    Node {
                        width: Val::Percent(100.0 / 10.0),
                        padding: UiRect::all(Val::Px(1.0)),
                        margin: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(2.0)),
                ))
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                });
            });
        },
    );
}
