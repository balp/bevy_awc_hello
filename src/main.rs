use std::sync::{Arc, Mutex};
use awc::{Client, ws, BoxedSocket};

use bevy::{prelude::*};
use bevy::app::App;
use bevy::prelude::CoreSet::PreUpdate;
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::runtime::Runtime;
use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
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
struct SyncChannel<T> {
    pub(crate) sender: Sender<T>,
    pub(crate) receiver: Receiver<T>,
}

impl<T> Default for SyncChannel<T> {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            sender: sender,
            receiver: receiver  ,
        }
    }
}

#[derive(Resource)]
struct WsClient {
    runtime: Runtime,
    network_events: SyncChannel<WsNetworkEvent>,
}

async fn server_stuff(network_events_sender: Sender<WsNetworkEvent>) {
    let client = Client::new();
    let client_mutex = Mutex::new(client);

    let my_client = client_mutex.lock().;
    let websockets_request = my_client.ws("ws://echo.websocket.org");
    let connection_future = websockets_request.connect();
    if let Ok(c) = connection_future.await {
        let (_resp, mut connection) = c;
        network_events_sender.send(WsNetworkEvent::Connected)
            .expect("Unable to send error message");
    } else {
        network_events_sender.send(WsNetworkEvent::Error(NetworkError::NotConnected))
            .expect("Unable to send error message");
    }
    ()
}

impl WsClient {
    pub fn connect(&mut self) {
        println!("connect and stuff");
        let network_events_sender = self.network_events.sender.clone();
        self.runtime.spawn(async move {
            server_stuff(network_events_sender).await;
        });
        
    }
    pub fn new() -> Self {
        let mut network_channel: SyncChannel<WsNetworkEvent> = SyncChannel::default();
        let mut connection_channel: SyncChannel<actix_codec::Framed<BoxedSocket, ws::Codec>> = SyncChannel::default();
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build runtime"),
            network_events: network_channel,
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



fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(MinimalPlugins)
        .add_plugin(WsClientPlugin)
        .add_system(hello_world)
        .run();
}


fn hello_world(
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ws_client: ResMut<WsClient>,
) {
    match app_state.0 {
        AppState::Disconnected => {
            println!("Start connect...");
            ws_client.connect();
            next_state.set(AppState::Connecting);
        },
        AppState::Connecting => {
            println!("Connecting...");
            //next_state.set(AppState::Connected);
        },
        AppState::Connected => {
            println!("Connected....");
            todo!()
        },
    }
    debug!("tick");
}
