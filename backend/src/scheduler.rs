use tokio::time::{interval, Duration};
use tracing::info;

pub fn start_scheduler() {
    tokio::spawn(async {
        let mut interval = interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            info!("Scheduler: running periodic update.");
        }
    });
}
