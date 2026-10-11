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
 * The wallet's calls to the meter (payment_proto, wallet only). Each answers
 * at once: a job's network work runs in the meter, and the wallet asks for
 * its result on later ticks (v1.4.14).
 */

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_payment_proto::{parse_job_id, reply_body, request, Account, ReplyError};
use nonos_payment_proto::{ACCOUNT_ROWS_MAX, ACCOUNT_ROW_LEN, EAGAIN, EIO, HDR_LEN};
use nonos_payment_proto::{OP_ACCOUNT, OP_JOB, OP_SET_BUDGET, SERVICE, STATUS_LEN};

/// The meter answers from memory.
const TIMEOUT_MS: u64 = 4_000;
const HEAD_LEN: usize = 1 + 20 + 16 + 16 + 8 + 16 + 2;
/// The largest job result: two transaction hashes.
const JOB_MAX: usize = 64;

pub(super) fn call(op: u16, body: &[u8], max: usize) -> Result<Vec<u8>, i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(EAGAIN);
    }
    let id = mk_time_millis() as u32;
    let tx = request(op, id, body);
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + max];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return Err(EAGAIN);
    }
    match reply_body(&rx[..n as usize], op, id) {
        Ok(b) => Ok(b.to_vec()),
        Err(ReplyError::Status(s)) => Err(s),
        Err(_) => Err(EIO),
    }
}

/// The meter's ACCOUNT page from channel `from`, or None when it did not answer.
pub fn account(from: u8) -> Option<Account> {
    let max = HEAD_LEN + ACCOUNT_ROWS_MAX * ACCOUNT_ROW_LEN;
    Account::parse(&call(OP_ACCOUNT, &[from], max).ok()?)
}

pub fn set_budget(body: &[u8]) -> Result<(), i32> {
    call(OP_SET_BUDGET, body, 0).map(|_| ())
}

/// Start a job (OPEN START, TOPUP or CLOSE): its id, or the meter's status.
pub fn start(op: u16, body: &[u8]) -> Result<u32, i32> {
    parse_job_id(&call(op, body, 4)?).ok_or(EIO)
}

/// JOB(id): EAGAIN while it runs, then its result once.
pub fn job(id: u32) -> Result<Vec<u8>, i32> {
    call(OP_JOB, &id.to_le_bytes(), JOB_MAX)
}
