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

//! The meter's DRAIN, for settle alone (payment_proto): LIST pages through
//! the newest signed voucher of each channel; SIGN has the meter build and
//! sign settle() from its own vouchers of the named channels.

use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_payment_proto::{reply_body, request, Drain, DrainRequest, DrainSign, ReplyError};
use nonos_payment_proto::{VoucherRecord, DRAIN_ROWS_MAX, HDR_LEN, OP_DRAIN, SERVICE};
use nonos_payment_proto::{STATUS_LEN, VOUCHER_RECORD_LEN};
use nonos_settle_proto::{EAGAIN, EINVAL, EIO};

/// The meter answers from memory and the keyring; it never waits on a route.
const TIMEOUT_MS: u64 = 30_000;
/// A signed settle() of twelve vouchers is under 6 KiB.
const RAW_MAX: usize = 8 * 1024;

fn call(body: &[u8], max: usize) -> Result<Vec<u8>, i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(EAGAIN);
    }
    let id = u32::try_from(mk_time_millis() & i64::from(u32::MAX)).unwrap_or(0);
    let tx = request(OP_DRAIN, id, body).ok_or(EINVAL)?;
    let mut rx = alloc::vec![0u8; (HDR_LEN + STATUS_LEN).checked_add(max).ok_or(EINVAL)?];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return Err(EAGAIN);
    }
    let got = usize::try_from(n).ok().and_then(|n| rx.get(..n)).unwrap_or(&[]);
    match reply_body(got, OP_DRAIN, id) {
        Ok(b) => Ok(b.to_vec()),
        Err(ReplyError::Status(s)) => Err(s),
        Err(_) => Err(EIO),
    }
}

/// Every channel's newest signed voucher; a meter paging past 256 rows is EIO.
pub fn vouchers() -> Result<Vec<VoucherRecord>, i32> {
    let mut all = Vec::new();
    for _ in 0..=u8::MAX {
        let from = u8::try_from(all.len()).map_err(|_| EIO)?;
        let body = DrainRequest::List { from }.encode().ok_or(EINVAL)?;
        let page =
            Drain::parse(&call(&body, 2 + DRAIN_ROWS_MAX * VOUCHER_RECORD_LEN)?).ok_or(EIO)?;
        let n = page.rows.len();
        all.extend(page.rows);
        if n == 0 || all.len() >= usize::from(page.total) {
            return Ok(all);
        }
    }
    Err(EIO)
}

/// The raw settle() the meter signed for `s`.
pub fn sign(s: &DrainSign) -> Result<Vec<u8>, i32> {
    call(&DrainRequest::Sign(s.clone()).encode().ok_or(EINVAL)?, RAW_MAX)
}
