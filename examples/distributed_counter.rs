/// Example: Distributed counter simulation
///
/// This example simulates a distributed system with 3 replicas that each
/// increment a counter independently, then synchronize their states.
///
/// Run with: cargo run --example distributed_counter

use crdts::counters::GCounter;
use crdts::traits::CvRDT;

fn main() {
    println!("=== Distributed Counter Example ===\n");

    // Create 3 replicas
    let mut replica1 = GCounter::new(0, 3);
    let mut replica2 = GCounter::new(1, 3);
    let mut replica3 = GCounter::new(2, 3);

    println!("Initial state:");
    println!("  Replica 1: {}", replica1.value());
    println!("  Replica 2: {}", replica2.value());
    println!("  Replica 3: {}", replica3.value());

    // Each replica increments independently (simulating concurrent operations)
    println!("\nReplica 1 increments 2 times...");
    replica1.increment();
    replica1.increment();

    println!("Replica 2 increments 3 times...");
    replica2.increment();
    replica2.increment();
    replica2.increment();

    println!("Replica 3 increments 1 time...");
    replica3.increment();

    println!("\nAfter concurrent increments:");
    println!("  Replica 1: {}", replica1.value());
    println!("  Replica 2: {}", replica2.value());
    println!("  Replica 3: {}", replica3.value());

    // Simulate network synchronization
    println!("\n--- Network Synchronization ---");

    println!("\nReplica 1 and 2 sync...");
    let replica1_synced = replica1.merge(&replica2);
    println!("  Replica 1 (after merge): {}", replica1_synced.value());

    println!("\nReplica 3 syncs with merged state...");
    let final_state = replica1_synced.merge(&replica3);
    println!("  Final state: {}", final_state.value());

    println!("\n✓ All replicas converged to: {}", final_state.value());
    println!("  (2 + 3 + 1 = 6 total increments)\n");
}
