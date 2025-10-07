use crate::traits::{CvRDT, ReplicaId};

/// G-Counter (Grow-only Counter)
///
/// A G-Counter is a state-based CRDT that implements a counter that can only be incremented.
/// It is based on a vector of counters, one per replica. Each replica can only increment its
/// own counter. The value of the G-Counter is the sum of all counters in the vector.
///
/// The G-Counter forms a monotonic semi-lattice where:
/// - The partial order is defined as: a ≤ b iff ∀i: a[i] ≤ b[i]
/// - The merge operation is element-wise maximum: merge(a, b)[i] = max(a[i], b[i])
/// - The bottom element is a vector of zeros
///
/// Properties:
/// - Merge is commutative: merge(a, b) = merge(b, a)
/// - Merge is associative: merge(merge(a, b), c) = merge(a, merge(b, c))
/// - Merge is idempotent: merge(a, a) = a
///
/// Paper: "A comprehensive study of Convergent and Commutative Replicated Data Types"
///        Shapiro et al., Section 3.1.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GCounter {
    replica_id: usize,
    counters: Vec<u64>,
}

impl GCounter {
    /// Creates a new G-Counter for a replica with the given ID
    ///
    /// # Arguments
    /// * `replica_id` - The unique identifier for this replica (must be < num_replicas)
    /// * `num_replicas` - The total number of replicas in the system
    ///
    /// # Example
    /// ```
    /// use crdts::counters::GCounter;
    ///
    /// let counter = GCounter::new(0, 3);
    /// assert_eq!(counter.value(), 0);
    /// ```
    pub fn new(replica_id: usize, num_replicas: usize) -> Self {
        assert!(replica_id < num_replicas, "replica_id must be less than num_replicas");

        // TODO: Initialize the GCounter
        // 1. Set the replica_id field
        // 2. Create a vector of counters with length num_replicas, all initialized to 0
        // Hint: You can use vec![0; num_replicas]

        todo!("Initialize GCounter")
    }

    /// Increments the local counter for this replica
    ///
    /// This operation only modifies the counter at index `self.replica_id`.
    /// This ensures that different replicas can increment concurrently without conflicts.
    ///
    /// # Example
    /// ```
    /// use crdts::counters::GCounter;
    ///
    /// let mut counter = GCounter::new(0, 3);
    /// counter.increment();
    /// assert_eq!(counter.value(), 1);
    /// counter.increment();
    /// assert_eq!(counter.value(), 2);
    /// ```
    pub fn increment(&mut self) {
        // TODO: Increment the counter at position self.replica_id
        // This should increase self.counters[self.replica_id] by 1

        todo!("Implement increment")
    }

    /// Returns the current value of the counter
    ///
    /// The value is computed as the sum of all counters in the vector.
    /// This represents the total number of increments across all replicas.
    ///
    /// # Example
    /// ```
    /// use crdts::counters::GCounter;
    ///
    /// let mut counter = GCounter::new(0, 2);
    /// counter.increment();
    /// counter.increment();
    /// assert_eq!(counter.value(), 2);
    /// ```
    pub fn value(&self) -> u64 {
        // TODO: Return the sum of all counters
        // Hint: You can use self.counters.iter().sum()

        todo!("Implement value")
    }
}

impl CvRDT for GCounter {
    /// Merges two G-Counters by taking the element-wise maximum
    ///
    /// For each position i in the counter vector, the merged counter contains
    /// max(self.counters[i], other.counters[i]). This ensures that the merge
    /// operation is:
    /// - Commutative: merge(a, b) = merge(b, a)
    /// - Associative: merge(merge(a, b), c) = merge(a, merge(b, c))
    /// - Idempotent: merge(a, a) = a
    ///
    /// # Panics
    /// Panics if the two counters have different lengths
    ///
    /// # Example
    /// ```
    /// use crdts::counters::GCounter;
    /// use crdts::traits::CvRDT;
    ///
    /// let mut counter1 = GCounter::new(0, 2);
    /// let mut counter2 = GCounter::new(1, 2);
    ///
    /// counter1.increment();  // counter1: [1, 0]
    /// counter2.increment();  // counter2: [0, 1]
    ///
    /// let merged = counter1.merge(&counter2);  // merged: [1, 1]
    /// assert_eq!(merged.value(), 2);
    /// ```
    fn merge(&self, other: &Self) -> Self {
        assert_eq!(
            self.counters.len(),
            other.counters.len(),
            "Cannot merge counters with different number of replicas"
        );

        // TODO: Implement merge by taking element-wise maximum
        // 1. Create a new vector that contains max(self.counters[i], other.counters[i]) for each i
        // 2. Return a new GCounter with this merged vector
        //
        // Hint: You can use zip() and map() to combine the two vectors
        // Example: self.counters.iter().zip(other.counters.iter()).map(|(a, b)| ...)
        //
        // Note: The replica_id of the merged counter should be self.replica_id

        todo!("Implement merge")
    }
}

impl ReplicaId for GCounter {
    fn replica_id(&self) -> usize {
        self.replica_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_counter() {
        let counter = GCounter::new(0, 3);
        assert_eq!(counter.value(), 0);
        assert_eq!(counter.replica_id(), 0);
    }

    #[test]
    #[should_panic(expected = "replica_id must be less than num_replicas")]
    fn test_invalid_replica_id() {
        GCounter::new(3, 3);
    }

    #[test]
    fn test_single_increment() {
        let mut counter = GCounter::new(0, 1);
        counter.increment();
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_multiple_increments() {
        let mut counter = GCounter::new(0, 1);
        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 3);
    }

    #[test]
    fn test_merge_no_conflict() {
        let mut counter1 = GCounter::new(0, 2);
        let mut counter2 = GCounter::new(1, 2);

        counter1.increment();
        counter2.increment();

        let merged = counter1.merge(&counter2);
        assert_eq!(merged.value(), 2);
    }

    #[test]
    fn test_merge_commutativity() {
        let mut counter1 = GCounter::new(0, 2);
        let mut counter2 = GCounter::new(1, 2);

        counter1.increment();
        counter1.increment();
        counter2.increment();

        let merged1 = counter1.merge(&counter2);
        let merged2 = counter2.merge(&counter1);

        assert_eq!(merged1.value(), merged2.value());
        assert_eq!(merged1, merged2);
    }

    #[test]
    fn test_merge_associativity() {
        let mut counter1 = GCounter::new(0, 3);
        let mut counter2 = GCounter::new(1, 3);
        let counter3 = GCounter::new(2, 3);

        counter1.increment();
        counter2.increment();
        counter2.increment();
        counter3.increment();
        counter3.increment();
        counter3.increment();

        let merged1 = counter1.merge(&counter2).merge(&counter3);
        let merged2 = counter1.merge(&counter2.merge(&counter3));

        assert_eq!(merged1.value(), merged2.value());
        assert_eq!(merged1, merged2);
    }

    #[test]
    fn test_merge_idempotence() {
        let mut counter = GCounter::new(0, 2);
        counter.increment();
        counter.increment();

        let merged = counter.merge(&counter);
        assert_eq!(merged.value(), counter.value());
        assert_eq!(merged, counter);
    }

    #[test]
    fn test_concurrent_increments() {
        // Simulate two replicas incrementing independently
        let mut counter1 = GCounter::new(0, 2);
        let mut counter2 = GCounter::new(1, 2);

        // Replica 0 increments 3 times
        counter1.increment();
        counter1.increment();
        counter1.increment();

        // Replica 1 increments 2 times
        counter2.increment();
        counter2.increment();

        // Merge both ways
        let merged1 = counter1.merge(&counter2);
        let merged2 = counter2.merge(&counter1);

        // Both should converge to 5
        assert_eq!(merged1.value(), 5);
        assert_eq!(merged2.value(), 5);
        assert_eq!(merged1, merged2);
    }

    #[test]
    fn test_merge_with_different_states() {
        let mut counter1 = GCounter::new(0, 3);
        let mut counter2 = GCounter::new(1, 3);
        let counter3 = GCounter::new(2, 3);

        counter1.increment();
        counter1.increment();

        counter2.increment();

        // counter3 stays at 0

        let merged12 = counter1.merge(&counter2);
        let final_merged = merged12.merge(&counter3);

        assert_eq!(final_merged.value(), 3);
    }

    #[test]
    #[should_panic(expected = "Cannot merge counters with different number of replicas")]
    fn test_merge_different_sizes() {
        let counter1 = GCounter::new(0, 2);
        let counter2 = GCounter::new(0, 3);

        counter1.merge(&counter2);
    }
}
