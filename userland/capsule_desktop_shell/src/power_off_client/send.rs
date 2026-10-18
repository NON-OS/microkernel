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

//! Sending the confirmed power off. It takes the confirmation itself, which
//! only the router's answer makes (state/power_off.rs), so nothing in the
//! shell can reach the call without one.

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::frame::{heard, request, Heard, HDR_LEN, SERVICE};
use crate::state::power_off::Confirmed;

/// Long enough for the service to refuse; a power off that works never
/// answers, and the session is wiped before this runs out.
const TIMEOUT_MS: u64 = 5_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sent {
    /// No service is registered as `power.off`; nothing was sent.
    Missing,
    /// Sent, and this came back while the system still runs.
    Heard(Heard),
}

pub fn send(_go: Confirmed, request_id: u32) -> Sent {
    let mut port = 0u32;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, ptr::null_mut());
    if rc < 0 || port == 0 {
        return Sent::Missing;
    }
    let tx = request(request_id);
    let mut rx = [0u8; HDR_LEN + 4];
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    Sent::Heard(heard(rc, &rx, request_id))
}
