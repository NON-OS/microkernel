// NONOS Operating System (AGPL-3.0-or-later)

//! The confirm value either side sends.

use super::super::group::LEN;
use crate::wpa::sha256::hmac_sha256_parts;

/// CN(KCK, send-confirm, scalar1, element1, scalar2, element2).
pub fn confirm(
    kck: &[u8; 32],
    send_confirm: u16,
    scalar1: &[u8; LEN],
    element1: &[u8; 2 * LEN],
    scalar2: &[u8; LEN],
    element2: &[u8; 2 * LEN],
) -> [u8; 32] {
    hmac_sha256_parts(kck, &[&send_confirm.to_le_bytes(), scalar1, element1, scalar2, element2])
}
