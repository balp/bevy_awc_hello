use bevy::app::App;
use bevy::{prelude::*, utils::Uuid};
use bevy::tasks::{IoTaskPool, TaskPool};
use tokio::{
    runtime::Runtime,
    runtime::Handle,
    time::sleep,
    time::Duration,
};

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
    loop {
        sleep(Duration::from_secs(2)).await;
        println!("Hey...");
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(MyRuntime::new())
        .add_startup_system(run_in_bg)
        .run();
}

fn run_in_bg(my_runtime: ResMut<MyRuntime>) {
    let pool = IoTaskPool::get();
    let handle = my_runtime.runtime.handle().clone();
    let results = pool.scope(|s| {
        s.spawn(async {
            handle.spawn(loopie()).await.expect("Unable to wait for task");
            ()
        })
    });
}
