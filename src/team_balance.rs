use crate::matchmaker::Player;
use itertools::Itertools;

pub fn balance_teams(
    players: Vec<Player>,
) -> (Vec<Player>, Vec<Player>, i32) {
    assert_eq!(
        players.len(),
        10,
        "Exactly 10 players required"
    );

    let mut best_diff = i32::MAX;

    let mut best_team_a: Vec<Player> = Vec::new();
    let mut best_team_b: Vec<Player> = Vec::new();

    let indices: Vec<usize> = (0..10).collect();

    for combo in indices.iter().combinations(5) {
        let mut team_a = Vec::new();
        let mut team_b = Vec::new();

        for i in 0..10 {
            if combo.contains(&&i) {
                team_a.push(players[i].clone());
            } else {
                team_b.push(players[i].clone());
            }
        }

        let avg_a =
            team_a.iter().map(|p| p.mmr).sum::<i32>() / 5;

        let avg_b =
            team_b.iter().map(|p| p.mmr).sum::<i32>() / 5;

        let diff = (avg_a - avg_b).abs();

        if diff < best_diff {
            best_diff = diff;
            best_team_a = team_a;
            best_team_b = team_b;
        }
    }

    (
        best_team_a,
        best_team_b,
        best_diff,
    )
}