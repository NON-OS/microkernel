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

//! One PAY v2 call to the meter, on payment_proto's wire. The reply is
//! believed only for this call, and only as one whole signed voucher.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_payment_proto::{reply_body, request, ReplyError, VoucherRecord};
use nonos_payment_proto::{
    EAGAIN, EINVAL, EIO, HDR_LEN, OP_PAY, SERVICE, STATUS_LEN, VOUCHER_RECORD_LEN,
};

/// The meter asks the market and the keyring before it answers.
const TIMEOUT_MS: u64 = 10_000;

/// The meter's signed voucher for `body`, a PayRequest, or its status.
pub fn request_payment(body: &[u8]) -> Result<Vec<u8>, i32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 {
        return Err(EAGAIN);
    }
    let id = mk_time_millis() as u32;
    let tx = request(OP_PAY, id, body).ok_or(EINVAL)?;
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + VOUCHER_RECORD_LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n <= 0 {
        return Err(EAGAIN);
    }
    let voucher = match reply_body(rx.get(..n as usize).unwrap_or(&[]), OP_PAY, id) {
        Ok(b) => b,
        Err(ReplyError::Status(s)) => return Err(s),
        Err(_) => return Err(EIO),
    };
    VoucherRecord::parse(voucher).ok_or(EIO)?;
    Ok(voucher.to_vec())
}
