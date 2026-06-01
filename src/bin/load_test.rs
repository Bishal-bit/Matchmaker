use rand::Rng;
use std::sync::Arc;
use std::time::Instant;
use tokio::task;

use matchmaker::matchmaker::{MatchMaker, Player};

const TOTAL_PLAYERS: usize = 10_000;
const CONCURRENT_BATCHES: usize = 100;

#[tokio::main]
async fn main() {
    println!("======================================");
    println!("Matchmaker Load Test");
    println!("======================================");

    let matchmaker = Arc::new(MatchMaker::new());

    let start = Instant::now();

    let players_per_batch =
        TOTAL_PLAYERS / CONCURRENT_BATCHES;

    let mut handles = Vec::new();

    for batch in 0..CONCURRENT_BATCHES {
        let mm = matchmaker.clone();

        let handle = task::spawn(async move {
            let mut rng = rand::thread_rng();

            for i in 0..players_per_batch {
                let id = (batch * players_per_batch + i)
                    as u64;

                let player = Player {
                    id,
                    mmr: rng.gen_range(1000..3000),
                    joined_at: Instant::now(),
                };

                mm.add_player(player);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let elapsed = start.elapsed();

    println!(
        "Injected {} players in {:.2?}",
        TOTAL_PLAYERS,
        elapsed
    );

    println!(
        "Injection Throughput: {:.2} players/sec",
        TOTAL_PLAYERS as f64
            / elapsed.as_secs_f64()
    );

    println!("\nCurrent Metrics:");
    matchmaker.print_stats();
}