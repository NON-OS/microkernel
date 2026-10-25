//! The address other wallets pay: version, spend key, X-Wing key. 1,249 bytes.
//! The byte layout is the prover's `note_seal::Payee::address` and must not drift from it.
//! Text form: prefix, then base32 of the bytes and a four-byte checksum.

#[path = "receiving_address_read.rs"]
mod read;

pub use read::parse_receiving_address;

use super::base32::encode;
use super::domain::RECEIVE_ADDRESS_TAG;
use super::receive::ENCAPSULATION_KEY_BYTES;

pub const RECEIVING_ADDRESS_VERSION: u8 = 0x01;
pub const RECEIVING_ADDRESS_BYTES: usize = 1 + 32 + ENCAPSULATION_KEY_BYTES;
const PREFIX: &str = "nox1";
const CHECK: usize = 4;

/// The address bytes for a spend key and an encapsulation key.
pub fn receiving_address(
    spend_pk: &[u64; 4],
    ek: &[u8; ENCAPSULATION_KEY_BYTES],
) -> [u8; RECEIVING_ADDRESS_BYTES] {
    let mut a = [0u8; RECEIVING_ADDRESS_BYTES];
    let (version, rest) = a.split_at_mut(1);
    version.copy_from_slice(&[RECEIVING_ADDRESS_VERSION]);
    let (keys, ek_slot) = rest.split_at_mut(32);
    for (slot, w) in keys.chunks_exact_mut(8).zip(spend_pk.iter()) {
        slot.copy_from_slice(&w.to_le_bytes());
    }
    ek_slot.copy_from_slice(ek);
    a
}

fn checksum(bytes: &[u8]) -> [u8; CHECK] {
    let mut h = blake3::Hasher::new();
    h.update(RECEIVE_ADDRESS_TAG);
    h.update(bytes);
    let mut out = [0u8; CHECK];
    out.copy_from_slice(h.finalize().as_bytes().get(..CHECK).unwrap_or(&[0; CHECK]));
    out
}

/// The address as text, prefix and checksum included.
pub fn receiving_address_text(a: &[u8; RECEIVING_ADDRESS_BYTES]) -> String {
    let mut bytes = a.to_vec();
    bytes.extend_from_slice(&checksum(a));
    let mut text = String::from(PREFIX);
    text.push_str(&encode(&bytes).unwrap_or_default());
    text
}
