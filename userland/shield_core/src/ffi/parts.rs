//! An address taken apart into its two public keys, the spend key a note commits
//! to as four little endian words and the X25519 key it is sealed under, so a screen
//! shows what an address is. The hex is written here so no shell gets the byte order wrong.

use crate::keys::Address;

/// The two keys an address carries, in hex, in the address's order.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AddressParts {
    pub spend_hex: String,
    pub view_hex: String,
}

impl AddressParts {
    /// Take an address apart. Public so a test asserts the byte order, which fails silently.
    pub fn of(address: &Address) -> AddressParts {
        let mut spend = [0u8; 32];
        for (slot, word) in spend.chunks_exact_mut(8).zip(address.spend_pk.iter()) {
            slot.copy_from_slice(&word.to_le_bytes());
        }
        AddressParts { spend_hex: hex(&spend), view_hex: hex(&address.view_pk) }
    }
}

/// Lower case hex, a byte at a time, so no dependency picks the case or separator.
fn hex(bytes: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in bytes {
        out.push(nibble(byte >> 4));
        out.push(nibble(byte & 0x0f));
    }
    out
}

/// One hex digit. The '?' fallback cannot happen, and would show a key as broken.
fn nibble(value: u8) -> char {
    char::from_digit(u32::from(value), 16).unwrap_or('?')
}
