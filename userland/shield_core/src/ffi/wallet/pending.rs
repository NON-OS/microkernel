//! A public send between its review and the owner's yes.

/// A reviewed transaction, held as the screen described it.
pub(super) struct Pending {
    pub(super) id: u64,
    pub(super) network: crate::evm::Network,
    /// The account the review was worked out for.
    pub(super) from: [u8; 20],
    pub(super) tx: crate::evm::tx::Eip1559,
    pub(super) made: std::time::Instant,
    /// For a deposit, the note it creates, stored before the deposit leaves.
    pub(super) note: Option<crate::notes::NotePlaintext>,
}
