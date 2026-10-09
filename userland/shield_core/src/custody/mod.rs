//! Custody: the phrase, the seed, and the seal the seed sits under at rest.
//! The wrapping key never leaves the platform keystore, so a copy of the vault
//! file without the device is a copy of nothing.

pub(crate) mod format;
pub(crate) mod guard;
mod imported;
mod kind;
mod mnemonic;
mod phrase;
pub(crate) mod secret;
mod vault;

pub use guard::HardwareGuard;
pub use imported::{parse_key, seed_of_key};
pub use mnemonic::{generate_phrase, phrase_to_seed};
pub use phrase::Phrase;
pub use secret::Seed;
pub use vault::{Opened, Vault};
