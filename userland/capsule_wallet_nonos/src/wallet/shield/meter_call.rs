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

/*
 * The meter's ACCOUNT (payment_proto op 5), which only the wallet may ask:
 * here, for the address a withdrawal to the meter pays.
 */

use alloc::vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_payment_proto::{reply_body, request, Account, ACCOUNT_ROWS_MAX, ACCOUNT_ROW_LEN};
use nonos_payment_proto::{HDR_LEN, OP_ACCOUNT, SERVICE, STATUS_LEN};

/// The meter answers from memory.
const TIMEOUT_MS: u64 = 4_000;
const HEAD_LEN: usize = 1 + 20 + 16 + 16 + 8 + 16 + 2;

/// The meter's first page of ACCOUNT, or None when it did not answer.
pub fn account() -> Option<Account> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return None;
    }
    let id = mk_time_millis() as u32;
    let tx = request(OP_ACCOUNT, id, &[0]);
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + HEAD_LEN + ACCOUNT_ROWS_MAX * ACCOUNT_ROW_LEN];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return None;
    }
    Account::parse(reply_body(&rx[..n as usize], OP_ACCOUNT, id).ok()?)
}
