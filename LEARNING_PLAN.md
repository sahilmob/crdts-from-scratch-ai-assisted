# CRDT Learning Plan - Incremental Implementation in Rust

This plan guides you through implementing Conflict-free Replicated Data Types (CRDTs) from foundational concepts to advanced structures.

## Learning Path

### Phase 1: Foundations (Weeks 1-2)
**Goal**: Understand the core principles of CRDTs and implement basic counters

#### Step 1: G-Counter (Grow-only Counter)
- **Paper**: "A comprehensive study of Convergent and Commutative Replicated Data Types" - Shapiro et al.
- **Concepts**: State-based CRDTs, monotonic semi-lattice, merge operation
- **Implementation**:
  - Vector of counters (one per replica)
  - Increment operation (increases local counter)
  - Merge operation (element-wise max)
  - Query operation (sum of all counters)
- **Tests**: Single replica increments, multi-replica merges, commutativity, associativity, idempotence

#### Step 2: PN-Counter (Positive-Negative Counter)
- **Concepts**: Combining CRDTs, increment and decrement
- **Implementation**:
  - Two G-Counters (positive and negative)
  - Increment/decrement operations
  - Value query (positive - negative)
  - Merge operation
- **Tests**: Mixed operations, concurrent increments/decrements, convergence

### Phase 2: Sets (Weeks 3-4)
**Goal**: Implement various set CRDTs with different semantics

#### Step 3: G-Set (Grow-only Set)
- **Concepts**: Set as a semi-lattice, add-only semantics
- **Implementation**:
  - HashSet-based storage
  - Add operation
  - Merge (union)
  - Contains query
- **Tests**: Add operations, convergence, idempotence

#### Step 4: 2P-Set (Two-Phase Set)
- **Concepts**: Tombstones, add-remove semantics
- **Implementation**:
  - Two G-Sets (added and removed)
  - Add and remove operations
  - Contains query (in added, not in removed)
  - Merge both sets
- **Tests**: Add-remove sequences, concurrent operations, remove wins semantics

#### Step 5: LWW-Element-Set (Last-Write-Wins Element Set)
- **Concepts**: Timestamps, conflict resolution via timestamps
- **Implementation**:
  - Map from elements to (timestamp, is_deleted) pairs
  - Add and remove with timestamps
  - Merge (keep entry with highest timestamp)
  - Bias (add-wins or remove-wins)
- **Tests**: Concurrent adds/removes, timestamp conflicts, bias behavior

#### Step 6: OR-Set (Observed-Remove Set)
- **Concepts**: Unique tags, causal history, add-wins semantics
- **Implementation**:
  - Map from elements to sets of unique tags
  - Add with unique tag
  - Remove with observed tags
  - Merge (union of tags, remove tombstones)
- **Tests**: Concurrent add-remove, causality, add-wins semantics

### Phase 3: Registers (Week 5)
**Goal**: Implement register CRDTs for storing values

#### Step 7: LWW-Register (Last-Write-Wins Register)
- **Concepts**: Single-value storage with timestamps
- **Implementation**:
  - Value and timestamp pair
  - Write operation
  - Merge (keep value with highest timestamp)
- **Tests**: Concurrent writes, timestamp ordering

#### Step 8: MV-Register (Multi-Value Register)
- **Concepts**: Vector clocks, concurrent values, causality
- **Implementation**:
  - Set of (value, version_vector) pairs
  - Write operation (replaces causally dominated values)
  - Merge (union, remove dominated)
- **Tests**: Concurrent writes, causal ordering, multiple concurrent values

### Phase 4: Maps and Compositions (Week 6)
**Goal**: Compose CRDTs to build complex structures

#### Step 9: OR-Map (Observed-Remove Map)
- **Concepts**: Map of CRDTs, per-key CRDT operations
- **Implementation**:
  - Map from keys to CRDTs (e.g., OR-Set keys to LWW-Registers)
  - Update, remove operations
  - Merge (merge per-key CRDTs)
- **Tests**: Nested updates, key removal, concurrent operations

### Phase 5: Sequences (Weeks 7-8)
**Goal**: Implement sequence CRDTs for text editing

#### Step 10: RGA (Replicated Growable Array)
- **Paper**: "Replicated abstract data types: Building blocks for collaborative applications" - Roh et al.
- **Concepts**: Linked list with unique IDs, causal ordering
- **Implementation**:
  - Linked list with tombstones
  - Insert with position and unique ID
  - Delete operation (mark tombstone)
  - Merge operation
- **Tests**: Concurrent inserts, deletes, convergence

#### Step 11: Causal Trees (Optional - Advanced)
- **Paper**: "Deep hypertext with embedded revision control implemented in regular expressions" - Brandy
- **Concepts**: Tree structure, causal relationships
- **Implementation**:
  - Tree nodes with parent references
  - Insert operation
  - Linearization algorithm
- **Tests**: Complex concurrent edits, tree traversal

### Phase 6: Advanced Topics (Week 9+)
**Goal**: Explore optimizations and advanced concepts

#### Step 12: Delta-CRDTs
- **Paper**: "Delta State Replicated Data Types" - Almeida et al.
- **Concepts**: Efficient state synchronization, deltas instead of full state
- **Implementation**:
  - Convert previous CRDTs to delta-based
  - Delta generation and application
- **Tests**: Delta efficiency, correctness

#### Step 13: Garbage Collection
- **Concepts**: Removing tombstones, causal stability
- **Implementation**:
  - Version vectors for causal stability
  - Tombstone removal algorithms
- **Tests**: Safe tombstone removal, no data loss

## Project Structure

```
crdts/
├── src/
│   ├── lib.rs
│   ├── traits.rs          # Common CRDT traits
│   ├── counters/
│   │   ├── mod.rs
│   │   ├── g_counter.rs
│   │   └── pn_counter.rs
│   ├── sets/
│   │   ├── mod.rs
│   │   ├── g_set.rs
│   │   ├── two_p_set.rs
│   │   ├── lww_set.rs
│   │   └── or_set.rs
│   ├── registers/
│   │   ├── mod.rs
│   │   ├── lww_register.rs
│   │   └── mv_register.rs
│   ├── maps/
│   │   ├── mod.rs
│   │   └── or_map.rs
│   └── sequences/
│       ├── mod.rs
│       ├── rga.rs
│       └── causal_tree.rs
├── tests/
│   ├── counter_tests.rs
│   ├── set_tests.rs
│   ├── register_tests.rs
│   ├── map_tests.rs
│   └── sequence_tests.rs
├── examples/
│   ├── distributed_counter.rs
│   ├── collaborative_set.rs
│   └── text_editor.rs
└── Cargo.toml
```

## Key Papers

1. **Main Reference**: "A comprehensive study of Convergent and Commutative Replicated Data Types" - Shapiro, Preguiça, Baquero, Zawirski (2011)
2. "Conflict-free Replicated Data Types" - Shapiro, Preguiça, Baquero, Zawirski (2011) - Tech Report
3. "A Conflict-Free Replicated JSON Datatype" - Kleppmann & Beresford (2017)
4. "Delta State Replicated Data Types" - Almeida, Shoker, Baquero (2016)

## Workflow for Each Step

1. **I provide**: Skeleton code with TODO comments, trait definitions, test stubs
2. **You implement**: Fill in the TODO sections
3. **You run**: `cargo test` to verify your implementation
4. **I review**: Check your implementation, provide feedback, explain concepts
5. **Move to next step** when tests pass and implementation is solid

## Getting Started

Ready to start with Step 1: G-Counter? I'll set up the Rust project and provide the skeleton code!
