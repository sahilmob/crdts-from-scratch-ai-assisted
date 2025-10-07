# Step 1: Implementing G-Counter

Welcome to your first CRDT implementation! 🎉

## What is a G-Counter?

A **G-Counter** (Grow-only Counter) is the simplest CRDT. It's a distributed counter that can only increase. The key insight is that each replica maintains its own counter, and the global value is the sum of all counters.

### Key Concepts

1. **State-based CRDT**: Each replica stores the full state (a vector of counters)
2. **Monotonic Semi-lattice**: The state can only "grow" (increase), never decrease
3. **Merge Operation**: Combining two states uses element-wise maximum
4. **Convergence**: All replicas that have seen the same updates will have the same value

### Why It Works

- Each replica only modifies its own counter → no conflicts!
- Merge takes the maximum of each counter → preserves all increments
- Idempotent: merging the same state multiple times is safe
- Commutative: order of merges doesn't matter
- Associative: grouping of merges doesn't matter

## Your Task

Open `src/counters/g_counter.rs` and implement 4 TODO sections:

### 1. `new()` function
Initialize a GCounter with:
- The given `replica_id`
- A vector of `num_replicas` counters, all set to 0

**Hint**: Use `vec![0; num_replicas]`

### 2. `increment()` function
Increment the counter at position `self.replica_id` by 1.

**Hint**: `self.counters[self.replica_id] += 1`

### 3. `value()` function
Return the sum of all counters in the vector.

**Hint**: `self.counters.iter().sum()`

### 4. `merge()` function in `impl CvRDT`
Create a new GCounter where each counter is the maximum of the corresponding counters from `self` and `other`.

**Hints**:
- Use `zip()` to pair up elements from both vectors
- Use `map()` to apply max to each pair
- Use `collect()` to create the new vector
- Don't forget to set the `replica_id` in the returned GCounter

**Example**:
```rust
let merged_counters: Vec<u64> = self.counters
    .iter()
    .zip(other.counters.iter())
    .map(|(a, b)| *a.max(b))
    .collect();

GCounter {
    replica_id: self.replica_id,
    counters: merged_counters,
}
```

## Testing Your Implementation

```bash
# Run all tests
cargo test

# Run only G-Counter unit tests
cargo test g_counter::tests

# Run integration tests
cargo test counter_tests

# Run the example (after implementation)
cargo run --example distributed_counter
```

## Expected Test Results

When your implementation is correct, you should see:
```
running 11 tests
test counters::g_counter::tests::test_invalid_replica_id - should panic ... ok
test counters::g_counter::tests::test_new_counter ... ok
test counters::g_counter::tests::test_single_increment ... ok
test counters::g_counter::tests::test_multiple_increments ... ok
test counters::g_counter::tests::test_merge_no_conflict ... ok
test counters::g_counter::tests::test_merge_commutativity ... ok
test counters::g_counter::tests::test_merge_associativity ... ok
test counters::g_counter::tests::test_merge_idempotence ... ok
test counters::g_counter::tests::test_concurrent_increments ... ok
test counters::g_counter::tests::test_merge_with_different_states ... ok
test counters::g_counter::tests::test_merge_different_sizes - should panic ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Understanding the Tests

### Basic Tests
- `test_new_counter`: Creates a counter and checks initial state
- `test_single_increment`: Increments once, checks value is 1
- `test_multiple_increments`: Increments 3 times, checks value is 3

### CRDT Properties Tests
- `test_merge_commutativity`: merge(a, b) = merge(b, a)
- `test_merge_associativity`: merge(merge(a, b), c) = merge(a, merge(b, c))
- `test_merge_idempotence`: merge(a, a) = a

### Distributed Scenarios
- `test_concurrent_increments`: Two replicas increment independently then merge
- `test_merge_with_different_states`: Multiple replicas with different states
- `test_merge_no_conflict`: Basic merge of two different replicas

## Common Mistakes

1. **Forgetting to return a value**: Make sure to replace `todo!()` with actual code that returns the right type
2. **Not using `self.replica_id`**: Each replica should only modify its own counter
3. **Using sum instead of max in merge**: The merge must use element-wise maximum, not sum
4. **Dereferencing**: Remember to dereference when using iterators (`*a` or `*b`)

## Questions to Consider

1. Why do we use maximum instead of sum in the merge operation?
2. What would happen if we used sum in merge?
3. Why can this counter only grow and not decrease?
4. How does this design avoid conflicts between replicas?

## Next Steps

Once all tests pass:
1. Run the example: `cargo run --example distributed_counter`
2. Experiment: Try modifying the example to simulate different scenarios
3. Let me know you're ready, and I'll review your implementation!
4. After review, we'll move to Step 2: PN-Counter (a counter that can decrement!)

Good luck! 🦀
