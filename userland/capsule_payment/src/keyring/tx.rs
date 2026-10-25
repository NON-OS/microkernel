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

//! An EIP-1559 transaction signed with the meter's own key (OP_SIGN_TX).
//! The meter encoded its `to` and calldata itself; the keyring signs only
//! for the key's owner, which is this process.

use alloc::vec::Vec;

use nonos_libc::mk_getpid;
use nonos_nox_meter::open::{sign_tx_request, TxRequest, OP_SIGN_TX};
use nonos_payment_proto::EIO;

use super::call::call_upto;

/// No transaction the meter signs comes near this: a settle() of twelve
/// vouchers is under 6 KiB.
const RAW_MAX: usize = 8 * 1024;

/// The raw signed transaction, 0x02 and its RLP.
pub(crate) fn sign_tx(key_id: u32, t: &TxRequest) -> Result<Vec<u8>, i32> {
    let raw = call_upto(OP_SIGN_TX, &sign_tx_request(mk_getpid(), key_id, t), RAW_MAX)?;
    if raw.first() != Some(&0x02) {
        return Err(EIO);
    }
    Ok(raw)
}
