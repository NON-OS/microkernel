//! A spend as the rules see it, and why the pool, the prover or the circuit would refuse it.

/// A spend as the wallet is about to prove it: two inputs in, two outputs out.
#[derive(Clone, Copy, Debug)]
pub struct Intent {
    /// The inputs' values and assets, and their leaf positions, which must differ.
    pub inputs: [(u64, u64, u64); 2],
    /// The outputs' values and assets.
    pub outputs: [(u64, u64); 2],
    pub public_amount: u64,
    pub fee: u64,
    pub asset_id: u64,
    pub clearing_price: u64,
    pub recipient: [u8; 20],
    /// Who is paid the fee: the relayer that submits the proof.
    pub fee_recipient: [u8; 20],
}

/// Why the pool, the prover or the circuit would refuse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntentRefusal {
    MixedAssets,
    SameInputTwice,
    Unbalanced,
    /// A value, public amount or fee above `MAX_VALUE`, or a price outside the field.
    OutOfRange,
    /// An address the pool's statement cannot carry.
    NotRepresentable,
    FeeRecipientWithoutFee,
    FeeWithoutFeeRecipient,
    /// A transfer that names a recipient or a price.
    TransferHasPublicLeg,
    RecipientRequired,
    FeeExceedsCap,
    FeeOffQuote,
}
