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

//! One call to the driver, bounded.

use nonos_libc::mk_ipc_call_timeout;

const CALL_TIMEOUT_MS: u64 = 1000;

/// The reply's length, or none when the driver did not answer.
pub(super) fn call(port: u32, tx: &[u8], rx: &mut [u8]) -> Option<usize> {
    let got = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        CALL_TIMEOUT_MS,
    );
    (got > 0).then_some(got as usize)
}
