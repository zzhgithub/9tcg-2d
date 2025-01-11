use crate::core::process::ProcessState;
use bevy::prelude::App;
use bevy_eventwork::NetworkMessage;
use bevy_eventwork::tcp::TcpProvider;
use serde::{Deserialize, Serialize};

#[deprecated]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActionEvent {
    pub action: ActionType,
}
#[deprecated]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ActionType {
    #[default]
    None,
    // 谁抽了几张卡
    Draw {
        who: usize,
        num: usize,
    },
    // 放置 从某个位置放置了 模因卡
    Set,
    // 费用
    Cost,
    // 卡片从 a 移动了b
    Move,
    // 询问是否要发动效果？
    Q,
    // 询问结果
    A,
    // 流程发生变化
    Process {
        from: ProcessState,
        to: ProcessState,
    },
}

// 谁 目标 操作 内容
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToServerMessage {
    pub debug_message: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToClientMessage {
    pub debug_message: String,
}

impl NetworkMessage for ToServerMessage {
    const NAME: &'static str = "tcg:ToServerMessage";
}

impl NetworkMessage for ToClientMessage {
    const NAME: &'static str = "tcg:ToClientMessage";
}

#[allow(unused)]
pub fn client_register_network_messages(app: &mut App) {
    use bevy_eventwork::AppNetworkMessage;

    // The client registers messages that arrives from the server, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_message::<ToClientMessage, TcpProvider>();
}

#[allow(unused)]
pub fn server_register_network_messages(app: &mut App) {
    use bevy_eventwork::AppNetworkMessage;

    // The server registers messages that arrives from a client, so that
    // it is prepared to handle them. Otherwise, an error occurs.
    app.listen_for_message::<ToServerMessage, TcpProvider>();
}
