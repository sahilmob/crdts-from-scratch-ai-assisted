/// Common trait for all state-based CRDTs (CvRDT - Convergent Replicated Data Type)
///
/// A state-based CRDT must satisfy:
/// 1. The state forms a semi-lattice with a partial order
/// 2. The merge operation computes the least upper bound (join)
/// 3. Merge is commutative, associative, and idempotent
pub trait CvRDT: Clone {
    /// Merge this CRDT with another, producing a new state that is the
    /// least upper bound of both states.
    ///
    /// Properties:
    /// - Commutative: a.merge(b) = b.merge(a)
    /// - Associative: a.merge(b).merge(c) = a.merge(b.merge(c))
    /// - Idempotent: a.merge(a) = a
    fn merge(&self, other: &Self) -> Self;
}

/// Common trait for operation-based CRDTs (CmRDT - Commutative Replicated Data Type)
///
/// An operation-based CRDT must satisfy:
/// 1. Operations are commutative
/// 2. Delivery of operations is exactly-once
/// 3. Causal delivery (optional, depends on implementation)
pub trait CmRDT {
    type Operation: Clone;

    /// Apply an operation to this CRDT
    fn apply(&mut self, op: Self::Operation);
}

/// Trait for CRDTs that support a unique identifier per replica
pub trait ReplicaId {
    /// Get the replica ID
    fn replica_id(&self) -> usize;
}
