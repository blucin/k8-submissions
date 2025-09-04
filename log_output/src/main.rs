use std::time::{Duration};
use std::thread::sleep;
use chrono::Local;
use uuid::Uuid;

fn main() {
    loop {
        let now = Local::now();
        let random_id = Uuid::new_v4();
        println!("{}: {}", now.format("%Y-%m-%dT%H:%M:%SZ"), random_id);
        sleep(Duration::new(5, 0));
    }
}
