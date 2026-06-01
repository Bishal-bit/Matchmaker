mod matchmaker;
mod team_balance;
mod metrics;

use matchmaker::{MatchMaker, Player};
use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("-----------------");
    println!("5v5 Real-Time Competitive Matchmaker");
    println!("-----------------");

    let matchmaker = Arc::new(MatchMaker::new());

    // Spawn matchmaking workers
    let worker_count = num_cpus::get();

    println!("Starting {} workers...", worker_count);

    for worker_id in 0..worker_count {
        let mm = matchmaker.clone();

        tokio::spawn(async move {
            mm.worker_loop(worker_id).await;
        });
    }

    // Simulate player arrivals
    simulate_players(matchmaker.clone(), 10000).await;

    // Let workers finish
    sleep(Duration::from_secs(10)).await;

    println!("\n===== Final Stats =====");
    matchmaker.print_stats();
}

async fn simulate_players(
    matchmaker: Arc<MatchMaker>,
    player_count: usize,
) {
    let mut rng = rand::thread_rng();

    println!(
        "Injecting {} players into matchmaking pool...",
        player_count
    );

    for id in 1..=player_count {
        let mmr = rng.gen_range(1000..3000);

        let player = Player {
            id: id as u64,
            mmr,
            joined_at: Instant::now(),
        };

        matchmaker.add_player(player);

        if id % 1000 == 0 {
            println!("Injected {} players", id);
        }

        sleep(Duration::from_millis(
            rng.gen_range(0..3),
        ))
        .await;
    }

    println!("Finished player injection.");
}