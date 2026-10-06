//! Which pending notes can be taken back. A hand-off never settled would hold them out of the
//! balance for good. A pending note whose nullifier is absent from a fresh history returns.
//! The pool refuses the second of two proofs, so a note still pays out once (`spec/Notes.tla`).

/// The pending notes to return: every one whose nullifier `spent` lacks.
pub(crate) fn releasable(pending: &[[u64; 4]], spent: &[[u64; 4]]) -> Vec<[u64; 4]> {
    pending.iter().filter(|cm| !spent.contains(cm)).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::releasable;

    #[test]
    fn only_a_pending_note_the_chain_has_not_spent_comes_back() {
        let (a, b, c) = ([1, 0, 0, 0], [2, 0, 0, 0], [3, 0, 0, 0]);
        assert_eq!(releasable(&[a, b], &[b, c]), vec![a], "b was settled after all");
        assert!(releasable(&[], &[a]).is_empty());
        assert_eq!(releasable(&[a], &[]), vec![a]);
    }
}
