# CRDTs in Rust - Learning by Implementation [AI-assisted]

A hands-on learning project for understanding Conflict-free Replicated Data Types (CRDTs) through incremental implementation in Rust.

## Getting Started

1. Read the [LEARNING_PLAN.md](LEARNING_PLAN.md) for the complete learning path
2. Start with Step 1: G-Counter
3. Implement the TODO sections in `src/counters/g_counter.rs`
4. Run tests with `cargo test`
5. When tests pass, move to the next step!

## Current Status

**Step 1: G-Counter** - Ready to implement!

### Your Task

Open `src/counters/g_counter.rs` and implement the three TODO sections:

1. `new()` - Initialize the GCounter with a vector of zeros
2. `increment()` - Increment the local replica's counter
3. `value()` - Return the sum of all counters
4. `merge()` - Merge two counters using element-wise max

### Running Tests

```bash
# Run all tests
cargo test

# Run only G-Counter tests
cargo test g_counter

# Run with output
cargo test -- --nocapture
```

### Verification

Your implementation is correct when all tests pass:

- ✅ Basic operations (new, increment, value)
- ✅ Merge properties (commutativity, associativity, idempotence)
- ✅ Distributed scenarios
- ✅ Eventual consistency

## Resources

- Main paper: "A comprehensive study of Convergent and Commutative Replicated Data Types" by Shapiro et al.
- [CRDT Tech Report](https://hal.inria.fr/inria-00555588/document)

## Project Structure

```
crdts/
├── src/
│   ├── lib.rs              # Library root
│   ├── traits.rs           # Common CRDT traits
│   └── counters/           # Counter CRDTs
│       ├── mod.rs
│       └── g_counter.rs    # ← Start here!
├── tests/
│   └── counter_tests.rs    # Integration tests
├── LEARNING_PLAN.md        # Full learning path
└── README.md              # This file
```

Good luck! 🦀
