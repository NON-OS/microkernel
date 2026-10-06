//! Which secret a vault seals, and the associated data that binds the seal to it.

use super::format::{AAD, AAD_KEY, AAD_WORDS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Seed,
    SeedAndWords,
    /// A private key brought in, with no words behind it.
    Key,
}

impl Kind {
    pub fn aad(self) -> &'static [u8] {
        match self {
            Kind::Seed => AAD,
            Kind::SeedAndWords => AAD_WORDS,
            Kind::Key => AAD_KEY,
        }
    }
}
