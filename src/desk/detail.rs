use crate::common::desks_datas::{DeskData, DesksDataList};
use crate::common::game_state::DeskState;
use crate::desk::desk_button_action::{DeskButtonActionState, DeskButtonActions};
use crate::desk::layout_back_button_and_content;
use bevy::prelude::*;
use bevy_persistent::Persistent;

#[derive(Debug, Resource)]
pub struct DeskSelect(pub Option<usize>);

pub fn open_desk_detail(
    mut commands: Commands,
    desk_list: Res<Persistent<DesksDataList>>,
    desk_select: Res<DeskSelect>,
    asset_server: Res<AssetServer>,
) {
    // 打开详情
    let list_array: &[DeskData] = &desk_list.list;
    //fixme
    let font = asset_server.load("fonts/wqy-microhei.ttc");
    layout_back_button_and_content(
        commands,
        font.clone(),
        DeskState::Detail,
        Box::from([
            ("返回", DeskButtonActions(DeskButtonActionState::BackToDesk)),
            ("Save", DeskButtonActions(DeskButtonActionState::NewDesk)),
        ]),
        |parent| {
            // 这里是测试
        },
    );
}
