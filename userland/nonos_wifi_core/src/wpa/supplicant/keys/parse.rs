// NONOS Operating System (AGPL-3.0-or-later)

//! The Key RSC, the key data parsed, and the stack wiped.

use super::pick::group_keys;
use super::store::GroupKeys;
use crate::eapol::kde::{parse_key_data, KeyData};

/// The packet number in a Key RSC field (little-endian, six octets used).
pub(in super::super) fn rsc_pn(rsc: &[u8; 8]) -> u64 {
    let mut b = [0u8; 8];
    b[..6].copy_from_slice(&rsc[..6]);
    u64::from_le_bytes(b)
}

/// Parse unwrapped key data and extract the group keys from it.
pub(in super::super) fn parse_group_keys(plain: &[u8]) -> Option<(KeyData<'_>, GroupKeys)> {
    let kd = parse_key_data(plain)?;
    let keys = group_keys(&kd)?;
    Some((kd, keys))
}

/// Overwrite secret bytes so they do not linger on the stack.
pub(in super::super) fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
