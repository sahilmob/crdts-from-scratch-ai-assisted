use crdts::counters::GCounter;
use crdts::traits::CvRDT;

/// Integration test: Simulating a distributed system with 3 replicas
#[test]
fn test_distributed_scenario() {
    // Create 3 replicas
    let mut replica1 = GCounter::new(0, 3);
    let mut replica2 = GCounter::new(1, 3);
    let mut replica3 = GCounter::new(2, 3);

    // Each replica increments independently
    replica1.increment();
    replica1.increment();

    replica2.increment();

    replica3.increment();
    replica3.increment();
    replica3.increment();

    // Simulate network synchronization: replica1 and replica2 sync
    let replica1_v2 = replica1.merge(&replica2);

    // Later, replica3 receives updates from replica1_v2
    let final_state = replica1_v2.merge(&replica3);

    // All increments should be counted
    assert_eq!(final_state.value(), 6);
}

/// Test eventual consistency: All replicas converge to same value
#[test]
fn test_eventual_consistency() {
    let mut replica1 = GCounter::new(0, 3);
    let mut replica2 = GCounter::new(1, 3);
    let mut replica3 = GCounter::new(2, 3);

    // Concurrent increments
    replica1.increment();
    replica2.increment();
    replica3.increment();

    // Different merge orders should produce same result
    let order1 = replica1.merge(&replica2).merge(&replica3);
    let order2 = replica2.merge(&replica3).merge(&replica1);
    let order3 = replica3.merge(&replica1).merge(&replica2);

    assert_eq!(order1.value(), 3);
    assert_eq!(order2.value(), 3);
    assert_eq!(order3.value(), 3);
    assert_eq!(order1, order2);
    assert_eq!(order2, order3);
}

/// Test that merge preserves monotonicity
#[test]
fn test_merge_monotonicity() {
    let mut replica1 = GCounter::new(0, 2);
    let replica2 = GCounter::new(1, 2);

    replica1.increment();
    replica1.increment();

    let initial_value = replica1.value();
    let merged = replica1.merge(&replica2);

    // After merge, value should be at least the initial value (monotonic)
    assert!(merged.value() >= initial_value);
}
