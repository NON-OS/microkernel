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

//! One NBTC call to driver.bt_usb0, found through the registry.

use core::ptr;

use nonos_bt_proto::{read_reply, request, HDR_LEN, REPLY_MAX};
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use crate::bluetooth::panel::{Panel, Reach};

const SERVICE: &[u8] = b"driver.bt_usb0";
/// The capsule answers between two turns of its link, each a millisecond
/// pause or one HCI exchange, so an answer is quick or not coming; the page
/// waits no longer than the policy store's calls do.
const CALL_TIMEOUT_MS: u64 = 500;

/// The status, and where the reply's payload ends in `rx`; it starts at
/// `DATA_AT`. The service unreached is the error, with the port forgotten,
/// so the next call asks the registry again: a restarted capsule has a new
/// port.
pub fn call(
    p: &mut Panel,
    op: u16,
    body: &[u8],
    rx: &mut [u8; REPLY_MAX],
) -> Result<(i32, usize), Reach> {
    let port = match p.port {
        Some(port) => port,
        None => lookup().ok_or(Reach::NoService)?,
    };
    p.port = Some(port);
    p.request_id = p.request_id.wrapping_add(1);
    let mut tx = [0u8; HDR_LEN + 8];
    let n = request(&mut tx, op, p.request_id, body).ok_or(Reach::NoAnswer)?;
    let got = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        n,
        rx.as_mut_ptr(),
        rx.len(),
        CALL_TIMEOUT_MS,
    );
    let end = usize::try_from(got).ok().filter(|&g| g <= rx.len()).unwrap_or(0);
    match read_reply(&rx[..end], op, p.request_id) {
        Some((status, _)) => Ok((status, end)),
        None => {
            p.port = None;
            Err(Reach::NoAnswer)
        }
    }
}

fn lookup() -> Option<u32> {
    let mut port: u32 = 0;
    let rc =
        mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port as *mut u32, ptr::null_mut());
    (rc == 0 && port != 0).then_some(port)
}
