# Matchmaker
# 5v5 Real-Time Competitive Matchmaker

A high-performance, thread-safe matchmaking engine written in Rust.

This project simulates a real-time multiplayer matchmaking service capable of handling thousands of concurrent player requests while balancing the trade-off between match quality and matchmaking latency.

---

# Features

- Thread-safe in-memory player pool
- Concurrent matchmaking workers
- Atomic player eviction
- Time-based MMR relaxation
- Balanced 5v5 team generation
- Lock-free metrics collection
- Load testing with 10,000+ simulated players

---

# Architecture

## Components

```text
Player Requests
       |
       v
+-------------------+
|   MatchMaker      |
|  (DashMap Pool)   |
+-------------------+
       |
       v
+-------------------+
| Worker Threads    |
+-------------------+
       |
       v
+-------------------+
| Team Balancer     |
+-------------------+
       |
       v
+-------------------+
| Match Created     |
+-------------------+
```

---

# Player Model

Each player contains:

```rust
pub struct Player {
    pub id: u64,
    pub mmr: i32,
    pub joined_at: Instant,
}
```

Where:

- `id` = unique player identifier
- `mmr` = matchmaking rating
- `joined_at` = queue entry timestamp

---

# Matchmaking Strategy

The primary objective is balancing:

1. Match Quality
2. Queue Time

These goals naturally conflict.

---

## Initial Matching Window

New players are matched within:

```text
±50 MMR
```

Example:

```text
Player MMR = 1500

Allowed:
1450 - 1550
```

---

## Time-Based Constraint Relaxation

To prevent indefinite waiting, the acceptable MMR range expands over time.

Formula:

```text
range = 50 + (wait_seconds × 5)
```

Examples:

| Wait Time | MMR Range |
|------------|------------|
| 0 sec | ±50 |
| 10 sec | ±100 |
| 20 sec | ±150 |
| 30 sec | ±200 |

This improves matchmaking latency for players in sparse skill brackets.

---

# Concurrent Worker Design

The service launches multiple matchmaking workers.

Worker count:

```rust
num_cpus::get()
```

Example:

```text
8 CPU cores
→ 8 matchmaking workers
```

Each worker:

1. Scans player pool
2. Finds candidate players
3. Attempts atomic eviction
4. Creates balanced match

---

# Thread Safety

The player pool is implemented using:

```rust
DashMap<u64, Player>
```

Benefits:

- Concurrent reads
- Concurrent writes
- No global mutex
- O(1) insert/remove

---

# Atomic Player Eviction

Multiple workers may discover the same players simultaneously.

Example:

```text
Worker A selects Player 101
Worker B selects Player 101
```

To avoid duplicate matches:

```rust
self.players.remove(&player_id)
```

is used as an atomic removal operation.

If any removal fails:

```text
Match creation aborted
Rollback executed
```

This guarantees a player appears in at most one match.

---

# Team Balance Optimization

Finding 10 compatible players is not enough.

The players must also be split fairly into two teams.

---

## Strategy

Given:

```text
10 players
```

All possible team splits are evaluated.

Number of combinations:

```text
10 choose 5 = 252
```

For each split:

```text
avg_team_a_mmr
avg_team_b_mmr
```

are calculated.

Balance score:

```text
abs(avg_team_a - avg_team_b)
```

The split with the minimum difference is selected.

---

## Example

Players:

```text
1400
1450
1500
1520
1550
1580
1600
1610
1630
1650
```

Balanced result:

```text
Team A Avg = 1549
Team B Avg = 1551

Difference = 2
```

---

# Metrics Collection

Metrics are implemented using:

```rust
AtomicU64
```

to avoid locking overhead.

Tracked metrics:

- Players Joined
- Players Matched
- Matches Created
- Average Wait Time
- Average Team MMR Difference

---

# Complexity Analysis

## Player Insert

```text
O(1)
```

---

## Player Removal

```text
O(1)
```

---

## Candidate Search

```text
O(n)
```

Current implementation scans the player pool.

Future optimization:

```text
BTreeMap
SkipList
MMR Buckets
```

---

## Team Balancing

```text
252 combinations
```

Effectively constant time:

```text
O(1)
```

because match size is fixed.

---

## Memory Usage

```text
O(n)
```

Where:

```text
n = waiting players
```

---

# Load Testing

The simulation generates:

```text
10,000 players
```

with random:

```text
MMR: 1000–3000
```

and injects them concurrently into the matchmaking engine.

Example output:

```text
Players Joined  : 10000
Players Matched : 9980
Matches Created : 998
Avg Wait Time   : 1.41 sec
Avg Team Diff   : 7.83 MMR
```

---

# Scaling Strategy

For larger deployments:

---

## Horizontal Sharding

Shard players by:

```text
MMR Range
```

Example:

```text
0–1000
1000–2000
2000–3000
```

Each shard runs independent workers.

---

## Distributed Matchmaking

Possible future architecture:

```text
Gateway
    |
    +---- Matchmaker Node 1
    |
    +---- Matchmaker Node 2
    |
    +---- Matchmaker Node 3
```

Cross-node matching is enabled only after sufficient wait time.

---

# Running

## Build

```bash
cargo build --release
```

---

## Run Matchmaker

```bash
cargo run --release
```

---

## Run Load Test

```bash
cargo run --bin load_test --release
```

---

# Technologies

- Rust
- Tokio
- DashMap
- AtomicU64
- Itertools

---

# Future Improvements

- Ranked queue priorities
- Party matchmaking
- Geographic region filtering
- Role-based matchmaking
- MMR bucket indexing
- Distributed queue synchronization

---

# Author

Bishal Sarkar

M.Tech Data Science
IIT Guwahati
