//! The receiving key: an X-Wing seed derived from the wallet seed, apart from the spend secret.
//! Notes stay sealed while either X25519 or ML-KEM-768 holds. The decapsulation key wipes on drop.

use super::derive::byte_key;
use super::domain::RECEIVE_CONTEXT;
use x_wing::{DecapsulationKey, Decapsulator, KeyExport};
use zeroize::Zeroizing;

/// Bytes in the X-Wing encapsulation key a sender needs.
pub const ENCAPSULATION_KEY_BYTES: usize = x_wing::ENCAPSULATION_KEY_SIZE;

/// This wallet's X-Wing receiving key.
pub struct ReceiveKey {
    dk: DecapsulationKey,
    /// The 32-byte X-Wing seed, kept only so an incoming view key can carry it.
    seed: Zeroizing<[u8; 32]>,
}

impl ReceiveKey {
    /// Derive the receiving key from the wallet seed.
    pub(crate) fn from_seed(seed: &[u8]) -> ReceiveKey {
        ReceiveKey::from_x_wing_seed(Zeroizing::new(byte_key(seed, RECEIVE_CONTEXT)))
    }

    /// The receiving key a view key carries.
    pub(crate) fn from_x_wing_seed(seed: Zeroizing<[u8; 32]>) -> ReceiveKey {
        ReceiveKey { dk: DecapsulationKey::from(*seed), seed }
    }

    pub(crate) fn x_wing_seed(&self) -> &[u8; 32] {
        &self.seed
    }

    /// The decapsulation key, for opening notes during a scan.
    pub(crate) fn dk(&self) -> &DecapsulationKey {
        &self.dk
    }

    /// The public half a sender seals to.
    pub fn encapsulation_key(&self) -> [u8; ENCAPSULATION_KEY_BYTES] {
        let mut out = [0u8; ENCAPSULATION_KEY_BYTES];
        out.copy_from_slice(&self.dk.encapsulation_key().to_bytes());
        out
    }
}
