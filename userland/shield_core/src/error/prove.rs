//! Failures on the proving path.

/// Failures on the proving path. A proof that does not verify on this device is never sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProveError {
    SeedEntropy,
    /// A blinding seed offered twice. Two proofs under one seed cancel to the witness.
    SeedReused,
    WitnessUnsatisfied,
    Unbalanced,
    SelfVerify,
    Cancelled,
    Encoding,
    /// The witness root is not the pool's published root, so the proof would be void.
    RootNotPublished,
}

impl core::fmt::Display for ProveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            ProveError::SeedEntropy => "the device random source did not yield a blinding seed",
            ProveError::SeedReused => "that blinding seed was already used",
            ProveError::WitnessUnsatisfied => "the transfer does not satisfy the circuit",
            ProveError::Unbalanced => "the amounts do not balance",
            ProveError::SelfVerify => "the proof did not verify on this device",
            ProveError::Cancelled => "proving was cancelled",
            ProveError::Encoding => "the proof could not be encoded",
            ProveError::RootNotPublished => "the pool moved while the proof was being made",
        })
    }
}

impl core::error::Error for ProveError {}
