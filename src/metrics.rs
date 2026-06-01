use std::sync::atomic::{AtomicU64, Ordering};

pub struct Metrics {
    pub players_joined: AtomicU64,
    pub players_matched: AtomicU64,
    pub matches_created: AtomicU64,

    pub total_wait_time: AtomicU64,
    pub total_balance_diff: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            players_joined: AtomicU64::new(0),
            players_matched: AtomicU64::new(0),
            matches_created: AtomicU64::new(0),

            total_wait_time: AtomicU64::new(0),
            total_balance_diff: AtomicU64::new(0),
        }
    }

    pub fn print(&self) {
        let joined =
            self.players_joined.load(Ordering::Relaxed);

        let matched =
            self.players_matched.load(Ordering::Relaxed);

        let matches =
            self.matches_created.load(Ordering::Relaxed);

        let total_wait =
            self.total_wait_time.load(Ordering::Relaxed);

        let total_diff =
            self.total_balance_diff.load(Ordering::Relaxed);

        let avg_wait = if matches > 0 {
            total_wait as f64 / matches as f64
        } else {
            0.0
        };

        let avg_balance_diff = if matches > 0 {
            total_diff as f64 / matches as f64
        } else {
            0.0
        };

        println!("--------------------------------");
        println!("Players Joined  : {}", joined);
        println!("Players Matched : {}", matched);
        println!("Matches Created : {}", matches);
        println!("Avg Wait Time   : {:.2} sec", avg_wait);
        println!(
            "Avg Team Diff   : {:.2} MMR",
            avg_balance_diff
        );
        println!("--------------------------------");
    }
}