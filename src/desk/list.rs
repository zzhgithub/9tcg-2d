use crate::common::game_state::DeskState;
use crate::common::test_data::ALL_CARD;
use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::desk::layout_back_button_and_content;
use crate::desk::scroll_list::scroll_list;
use crate::utils::preview_plugins::ImagePreview;
use bevy::prelude::*;

pub fn list_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/wqy-microhei.ttc");

    layout_back_button_and_content(
        commands,
        font,
        DeskState::List,
        Box::from([("返回", DeskButtonActions(DeskButtonActionState::BackToMenu))]),
        |plane| {
            //FIXME: 这里是测试使用的代码
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
                    scroll_list(parent, &ALL_CARD, 10, |row, &t| {
                        let image = asset_server.load(format!("cards/{}.png", t));
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
                            ImagePreview(image.clone()),
                        ))
                        .insert(PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        });
                    });
                });
        },
    );
}
