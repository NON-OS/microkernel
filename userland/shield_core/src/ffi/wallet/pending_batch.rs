//! Several deposits between their review and the yes of the owner, each held with its note.

pub(super) struct PendingBatch {
    pub(super) id: u64,
    pub(super) from: [u8; 20],
    pub(super) steps: Vec<(crate::evm::tx::Eip1559, crate::notes::NotePlaintext)>,
    pub(super) made: std::time::Instant,
}
