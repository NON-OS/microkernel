//! Failures on the custody path. Variants name a stage, and none carries key material,
//! a phrase or a byte count.

/// Failures on the custody path: entropy, mnemonic, seal and the hardware guard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustodyError {
    Entropy,
    Mnemonic,
    PhraseLength,
    /// The sealed seed did not authenticate: wrong key, or the file moved.
    SealAuth,
    Guard,
    VaultShape,
    /// A vault already exists here, and overwriting would destroy the only seed.
    VaultPresent,
    Locked,
    /// A private key that is not 64 hex digits of a key on the curve.
    KeyShape,
}

impl core::fmt::Display for CustodyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            CustodyError::Entropy => "the platform random source refused",
            CustodyError::Mnemonic => "the recovery phrase did not check out",
            CustodyError::PhraseLength => "the recovery phrase is not 12, 15, 18, 21 or 24 words",
            CustodyError::SealAuth => "the stored seed did not authenticate",
            CustodyError::Guard => "the hardware keystore refused",
            CustodyError::VaultShape => "the stored wallet is not readable",
            CustodyError::VaultPresent => "a wallet is already stored here",
            CustodyError::Locked => "the wallet is locked",
            CustodyError::KeyShape => "that private key is not valid",
        })
    }
}

impl core::error::Error for CustodyError {}
