use crate::common::desks_datas::DeskData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ToServerAction {
    JoinRoom(JoinRoomData),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinRoomData {
    pub username: String,
    pub room_name: String,
    pub desk: DeskData,
}
