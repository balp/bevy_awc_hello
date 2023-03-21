use tokio::time::{sleep, Duration};


async fn loopie() {
    loop {
        sleep(Duration::from_secs(2)).await;
        println!("Hey...");
    }
}

#[tokio::main]
async fn main() {
    loopie().await;
}