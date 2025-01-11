use async_net::Ipv4Addr;
use bevy::tasks::TaskPool;
use bevy::{prelude::*, tasks::TaskPoolBuilder};
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network, NetworkData, NetworkEvent};
use std::net::{IpAddr, SocketAddr};

use bevy_eventwork::tcp::{NetworkSettings, TcpProvider};
use tcg_2d::core::action_event::{ToServerMessage, server_register_network_messages};

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
    app.run();
}

fn setup_networking(
    mut net: ResMut<Network<TcpProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    match net.listen(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7000),
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
    mut commands: Commands,
    net: Res<Network<TcpProvider>>,
    mut network_events: EventReader<NetworkEvent>,
) {
    for event in network_events.read() {
        match event {
            //todo 连接成功时和失败时逻辑
            NetworkEvent::Connected(connect_id) => {
                info!("New connection: {}", connect_id);
            }
            NetworkEvent::Disconnected(connect_id) => {
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
) {
    for message in new_messages.read() {
        info!("Received message: {:?}", message);
    }
}
