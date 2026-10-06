//! What a deposit from the public account hands the screen, amounts already formatted.

/// A deposit worked out and waiting for a yes: the approval first, or the deposit itself.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicShield {
    /// The review to confirm, or 0 with a refusal.
    pub id: u64,
    pub refusal: Option<String>,
    /// True when this is the exact approval of the pool, and the deposit follows after it.
    pub approval: bool,
    pub amount: String,
    pub pool_fee: String,
    /// What the shielded note holds after the pool fee.
    pub shielded: String,
    pub max_network_fee: String,
    pub valid_for_seconds: u32,
}
