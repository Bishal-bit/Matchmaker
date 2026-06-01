use crate::metrics::Metrics;
use crate::team_balance::balance_teams;

use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Clone)]
pub struct Player {
    pub id: u64,
    pub mmr: i32,
    pub joined_at: Instant,
}

pub struct MatchMaker {
    pub players: DashMap<u64, Player>,
    pub metrics: Arc<Metrics>,
}

impl MatchMaker {
    pub fn new() -> Self {
        Self {
            players: DashMap::new(),
            metrics: Arc::new(Metrics::new()),
        }
    }

    pub fn add_player(&self, player: Player) {
        self.players.insert(player.id, player);
        self.metrics.players_joined.fetch_add(
            1,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    fn allowed_range(player: &Player) -> i32 {
        let wait_secs =
            player.joined_at.elapsed().as_secs() as i32;

        50 + (wait_secs * 5)
    }

    fn find_candidate_group(
        &self,
        seed_player: &Player,
    ) -> Option<Vec<Player>> {
        let range = Self::allowed_range(seed_player);

        let min_mmr = seed_player.mmr - range;
        let max_mmr = seed_player.mmr + range;

        let mut candidates = Vec::new();

        for entry in self.players.iter() {
            let p = entry.value();

            if p.mmr >= min_mmr && p.mmr <= max_mmr {
                candidates.push(p.clone());

                if candidates.len() >= 10 {
                    break;
                }
            }
        }

        if candidates.len() == 10 {
            Some(candidates)
        } else {
            None
        }
    }

    fn try_evict_players(
        &self,
        players: &[Player],
    ) -> bool {
        let mut removed = Vec::new();

        for p in players {
            if let Some((_, player)) =
                self.players.remove(&p.id)
            {
                removed.push(player);
            } else {
                // rollback
                for player in removed {
                    self.players.insert(
                        player.id,
                        player,
                    );
                }

                return false;
            }
        }

        true
    }

    fn create_match(
        &self,
        players: Vec<Player>,
    ) {
        let (team_a, team_b, diff) =
            balance_teams(players.clone());

        let avg_wait = players
            .iter()
            .map(|p| p.joined_at.elapsed().as_secs())
            .sum::<u64>()
            / players.len() as u64;

        self.metrics.matches_created.fetch_add(
            1,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.metrics.players_matched.fetch_add(
            10,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.metrics.total_wait_time.fetch_add(
            avg_wait,
            std::sync::atomic::Ordering::Relaxed,
        );

        self.metrics.total_balance_diff.fetch_add(
            diff as u64,
            std::sync::atomic::Ordering::Relaxed,
        );

        println!(
            "[MATCH] TeamA={} TeamB={} BalanceDiff={}",
            team_a.len(),
            team_b.len(),
            diff
        );
    }

    pub async fn worker_loop(
        &self,
        worker_id: usize,
    ) {
        loop {
            let snapshot: Vec<Player> = self
                .players
                .iter()
                .map(|p| p.value().clone())
                .collect();

            if snapshot.len() < 10 {
                sleep(Duration::from_millis(20)).await;
                continue;
            }

            for player in snapshot {
                let candidates =
                    self.find_candidate_group(&player);

                if candidates.is_none() {
                    continue;
                }

                let candidates =
                    candidates.unwrap();

                if self.try_evict_players(
                    &candidates,
                ) {
                    self.create_match(candidates);

                    println!(
                        "[Worker {}] Match created",
                        worker_id
                    );

                    break;
                }
            }

            sleep(Duration::from_millis(5)).await;
        }
    }

    pub fn print_stats(&self) {
        self.metrics.print();
    }
}