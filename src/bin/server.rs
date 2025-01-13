use async_net::Ipv4Addr;
use bevy::tasks::TaskPool;
use bevy::{prelude::*, tasks::TaskPoolBuilder};
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};
use std::net::{IpAddr, SocketAddr};

use bevy_eventwork::tcp::{NetworkSettings, TcpProvider};
use tcg_2d::core::action_event::{
    ToServerAction, ToServerMessage, server_register_network_messages,
};
use tcg_2d::core::duel::Duel;
use tcg_2d::server::{Player, PlayerState, PlayersManager, RoomManager};

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::log::LogPlugin::default()));

    app.add_plugins(bevy_eventwork::EventworkPlugin::<
        TcpProvider,
        bevy::tasks::TaskPool,
    >::default());

    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    // A good way to ensure that you are not forgetting to register
    // any messages is to register them where they are defined!
    server_register_network_messages(&mut app);

    app.add_systems(Startup, setup_networking);
    app.add_systems(Update, (handle_connection_events, handle_messages));

    app.insert_resource(NetworkSettings::default());

    app.insert_resource(PlayersManager::default());
    app.insert_resource(RoomManager::default());
    app.run();
}

fn setup_networking(
    mut net: ResMut<Network<TcpProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    match net.listen(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7001),
        &task_pool.0,
        &settings,
    ) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }

    info!("Started listening for new connections!");
}

fn handle_connection_events(
    mut network_events: EventReader<NetworkEvent>,
    mut players_manager: ResMut<PlayersManager>,
    mut room_manager: ResMut<RoomManager>,
) {
    for event in network_events.read() {
        match event {
            //todo 连接成功时和失败时逻辑
            NetworkEvent::Connected(connect_id) => {
                info!("New connection: {}", connect_id);
                players_manager.0.insert(connect_id.id, Player {
                    connect_id: connect_id.clone(),
                    state: PlayerState::Idle,
                });
            }
            NetworkEvent::Disconnected(connect_id) => {
                if let Some(player) = players_manager.0.remove(&connect_id.id) {
                    match player.state {
                        PlayerState::Idle => {
                            info!("remove player: {}", player.connect_id);
                        }
                        PlayerState::InRoom(room_name) => {
                            if let Some(duel) = room_manager.0.remove(&room_name) {
                                // todo 通知销毁了 房间
                                info!("通知 房间其他玩家房间销毁");
                            }
                        }
                    }
                }
                info!("lost connection: {}", connect_id);
            }
            NetworkEvent::Error(error) => {
                error!("Network error: {:?}", error);
            }
        }
    }
}

fn handle_messages(
    mut new_messages: EventReader<NetworkData<ToServerMessage>>,
    net: Res<Network<TcpProvider>>,
    mut players_manager: ResMut<PlayersManager>,
    mut room_manager: ResMut<RoomManager>,
) {
    for message in new_messages.read() {
        info!("Received message: {:?}", message);
        match message.action.clone() {
            ToServerAction::JoinRoom(data) => {
                let mut duel = match room_manager.0.get_mut(&data.room_name) {
                    None => Duel::default(),
                    Some(duel) => duel.clone(),
                };

                match duel.add_player(
                    data.username.clone(),
                    message.my_connect_id.clone(),
                    data.desk.clone(),
                ) {
                    Ok(_) => {
                        if duel.check_is_ready_to_play() {
                            duel.process();
                            // todo 这里进行之后的事件提醒
                        }
                    }
                    Err(_) => {
                        //TODO 通知下游出现问题
                        info!("Could not add player: {}", data.room_name.clone());
                        // 终止逻辑
                        return;
                    }
                }
                room_manager.0.insert(data.room_name.clone(), duel);
                if let Some(mut player) = players_manager.0.get_mut(&message.my_connect_id) {
                    player.state = PlayerState::InRoom(data.room_name.clone());
                }
            }
        }
    }
}
