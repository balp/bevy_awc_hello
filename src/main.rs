use bevy::app::App;
use bevy::{prelude::*, utils::Uuid};
use awc::{Client, ws};
use bevy::prelude::CoreSet::PreUpdate;
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::{
    runtime::Runtime,
};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_system(hello_world)
        .run();
}

async fn ws_connect() {
    let (_resp, mut connection) = Client::new()
        .ws("ws://echo.websocket.org")
        .connect()
        .await
        .unwrap();

    connection
        .send(ws::Message::Text("Echo".into()))
        .await
        .unwrap();
    let response = connection.next().await.unwrap().unwrap();

    assert_eq!(response, ws::Frame::Text("Echo".as_bytes().into()));
}

#[derive(Resource)]
struct WsClient {
    runtime: Runtime,
}

impl WsClient {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build runtime"),
        }
    }
}


#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("An error occured when accepting a new connnection: {0}")]
    Accept(std::io::Error),
    #[error("Not connected to any server")]
    NotConnected,
    #[error("An error occured when trying to start listening for new connections: {0}")]
    Listen(std::io::Error),
    #[error("An error occured when trying to connect: {0}")]
    Connection(std::io::Error),
}

#[derive(Debug)]
pub enum WsNetworkEvent {
    Connected,
    Disconnected,
    Error(NetworkError),
}

#[derive(Clone, Debug, Resource)]
pub struct WsNetworkSettings {
    pub max_packet_length: usize,
}

impl Default for WsNetworkSettings {
    fn default() -> Self {
        Self { max_packet_length: 10 * 1024 * 1024 }
    }
}

struct WsClientPlugin;

impl Plugin for WsClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WsClient::new())
            .add_event::<WsNetworkEvent>()
            .init_resource::<WsNetworkSettings>()
            .add_system((send_client_network_events).in_base_set(PreUpdate))
            .add_system((handle_connection_event).in_base_set(PreUpdate))
        ;
    }
}

fn send_client_network_events(
    client_server: ResMut<WsClient>,
    mut client_network_events: EventWriter<WsNetworkEvent>,
) {
    // client_network_events.send_batch(client_server.network_events.receiver.try_iter());
}

fn handle_connection_event(
    mut net_res: ResMut<WsClient>,
    mut events: EventWriter<WsNetworkEvent>,
) {}

fn hello_world() {
    debug!("tick");
}