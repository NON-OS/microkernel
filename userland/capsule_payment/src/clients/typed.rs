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

//! A voucher signed by the meter's own key, through the keyring's
//! OP_SIGN_TYPED_DATA entry 1. The keyring's table holds the type and the
//! domain; the reply is believed only when its signer is the meter and its
//! struct hash is the voucher's, computed here.

use nonos_libc::mk_getpid;
use nonos_nox_meter::typed::{parse_reply, voucher_request, OP_SIGN_TYPED_DATA, REPLY_LEN};
use nonos_nox_meter::voucher::Voucher;
use nonos_payment_proto::EIO;

use crate::keccak::Host;
use crate::keyring;

pub(crate) fn sign_voucher(key_id: u32, chain_id: u64, v: &Voucher) -> Result<[u8; 65], i32> {
    let tx = voucher_request(mk_getpid(), key_id, chain_id, v);
    let rx = keyring::call(OP_SIGN_TYPED_DATA, &tx, REPLY_LEN)?;
    let s = parse_reply(&rx[8..]).ok_or(EIO)?;
    if s.signer != v.payer || s.struct_hash != v.struct_hash(&Host) {
        return Err(EIO);
    }
    Ok(s.sig)
}
