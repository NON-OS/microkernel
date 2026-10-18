//! An address as the statement's four limbs: 48, 48, 48 and 16 bits, lowest first.

use super::MASK48;

/// An address as the statement's four limbs, lowest first.
pub fn address_limbs(addr: &[u8; 20]) -> [u64; 4] {
    let (high, low) = split(addr);
    let limb = |v: u128| u64::try_from(v & MASK48).unwrap_or(0);
    [
        limb(low),
        limb(low >> 48),
        limb((low >> 96) | (u128::from(high & 0xFFFF) << 32)),
        u64::from(high >> 16),
    ]
}

/// An address as its top 32 bits and low 128 bits.
fn split(addr: &[u8; 20]) -> (u32, u128) {
    let mut hi = [0u8; 4];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&addr[..4]);
    lo.copy_from_slice(&addr[4..]);
    (u32::from_be_bytes(hi), u128::from_be_bytes(lo))
}
