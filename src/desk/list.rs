use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::desk::layout_back_button_and_content;
use bevy::prelude::*;

pub fn list_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    layout_back_button_and_content(
        commands,
        font,
        DeskButtonActions(DeskButtonActionState::BackToMenu),
        |plane| {
            //do nothing
        },
    );
}
