use bevy::app::App;
use bevy::{prelude::*};
use bevy::tasks::{IoTaskPool, Task};
use tokio::{
    runtime::Runtime,
//    runtime::Handle,
    time::sleep,
    time::Duration,
};
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};


#[derive(Resource)]
struct MyRuntime {
    runtime: Runtime,
}

impl MyRuntime {
    pub fn new() -> Self {
        Self { runtime: Runtime::new().unwrap() }
    }
}

async fn loopie() {
    for i in 0..10 {
        sleep(Duration::from_secs(2)).await;
        println!("Hey: {}", i);
    }
}

async fn ws_stuff() {
    for i in 0..10 {
        sleep(Duration::from_secs(2)).await;
        println!("ws: {}", i);
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(MyRuntime::new())
        .add_startup_system(run_in_bg)
        .add_startup_system(get_ws_echo_tungstenite)
        .add_system(handle_loopie)
        .add_system(handle_tungstenite)
        .run();
}

#[derive(Component)]
struct LoopieTransform(Task<()>);

fn run_in_bg(mut commands: Commands, my_runtime: ResMut<MyRuntime>) {
    let pool = IoTaskPool::get();
    let handle = my_runtime.runtime.handle().clone();
    let loopie_thread = pool.spawn(async move {
        println!("spawn: loopie");
        handle.spawn(loopie()).await.expect("Unable to wait for task");
        println!("spawn: exit loopie");
        ()
    });
    commands.spawn(LoopieTransform(loopie_thread));
}

fn handle_loopie(
    mut commands: Commands,
    mut loopis_tasks: Query<(Entity, &mut LoopieTransform)>,
) {
    let pool = IoTaskPool::get();

    for (entity, mut task) in &mut loopis_tasks {
        // dbg!(entity);
        // dbg!(task);
    }
}

#[derive(Component)]
struct TungstenTransform(Task<()>);

fn get_ws_echo_tungstenite(
    mut commands: Commands,
    mut ws_tasks: Query<(Entity, &mut TungstenTransform)>,
    my_runtime: ResMut<MyRuntime>,
) {
    let pool = IoTaskPool::get();
    let handle = my_runtime.runtime.handle().clone();
    let ws_thread = pool.spawn(async move {
        println!("spawn: enter get_ws_echo_tungstenite");
        handle.spawn(ws_stuff()).await.expect("Unable to wait for task");
        println!("spawn: exit get_ws_echo_tungstenite");
    });
    commands.spawn(TungstenTransform(ws_thread));

}


fn handle_tungstenite(
    mut commands: Commands,
    mut loopis_tasks: Query<(Entity, &mut LoopieTransform)>,
) {
    let pool = IoTaskPool::get();

    for (entity, mut task) in &mut loopis_tasks {
        // dbg!(entity);
        // dbg!(task);
    }
}
