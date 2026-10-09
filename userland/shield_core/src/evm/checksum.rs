//! The EIP-55 spelling of an address, whose letter case carries a checksum.

use sha3::{Digest, Keccak256};

/// An address with its EIP-55 checksum casing, as wallets print it.
pub fn checksummed(address: &[u8; 20]) -> String {
    let lower: String = address.iter().map(|b| format!("{b:02x}")).collect();
    let hash = Keccak256::digest(lower.as_bytes());
    let mut out = String::from("0x");
    for (i, c) in lower.chars().enumerate() {
        let nibble = hash.get(i >> 1).map_or(0, |b| if i & 1 == 0 { b >> 4 } else { b & 0x0f });
        out.push(if c.is_ascii_alphabetic() && nibble >= 8 { c.to_ascii_uppercase() } else { c });
    }
    out
}
