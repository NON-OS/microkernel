// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! The EIP-1559 encodings of one parsed transaction: the unsigned form whose
//! keccak is signed, and the signed form the wallet broadcasts.

use alloc::vec;
use alloc::vec::Vec;

use super::super::eip1559::tx_fields;
use super::super::rlp::{rlp_list, rlp_uint_be};
use super::sign_tx_parse::Tx;

fn fields(t: &Tx<'_>) -> Vec<Vec<u8>> {
    tx_fields(&t.chain_be, &t.nonce, &t.max_priority, &t.max_fee, &t.gas, &t.to, &t.value, t.data)
}

/// keccak256(0x02 || rlp(fields)), or None if the keccak call fails.
pub(super) fn digest(t: &Tx<'_>) -> Option<[u8; 32]> {
    let mut unsigned = vec![0x02u8];
    unsigned.extend_from_slice(&rlp_list(&fields(t)));
    let mut digest = [0u8; 32];
    let n =
        nonos_libc::crypto_keccak256(unsigned.as_ptr(), unsigned.len(), digest.as_mut_ptr(), 32);
    (n == 32).then_some(digest)
}

/// 0x02 || rlp(fields, yParity, r, s) for a 65-byte signature with v 27 or 28.
pub(super) fn signed(t: &Tx<'_>, sig: &[u8; 65]) -> Vec<u8> {
    let mut f = fields(t);
    f.push(rlp_uint_be(&[sig[64] - 27]));
    f.push(rlp_uint_be(&sig[0..32]));
    f.push(rlp_uint_be(&sig[32..64]));
    let mut raw = vec![0x02u8];
    raw.extend_from_slice(&rlp_list(&f));
    raw
}
