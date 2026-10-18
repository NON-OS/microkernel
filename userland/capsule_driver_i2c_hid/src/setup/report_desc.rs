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

//! The report descriptor named in the HID descriptor, read whole.

use crate::i2c_client::write_read;
use crate::state::State;

/// Upper bound on the report descriptor we will read and parse. Real precision
/// touchpads (Synaptics, Elan) ship descriptors of several hundred bytes and up
/// to roughly a kilobyte, so a smaller cap silently drops the absolute-touch
/// layout. It is also the controller's read limit (TRANSFER_READ_MAX).
const REPORT_DESC_MAX: usize = 1024;
const REPORT_DESC_ATTEMPTS: u32 = 3;
const REPORT_DESC_RETRY_MS: u64 = 20;

/// The report descriptor, read whole. A device just out of RESET can still
/// be busy, so a failed or short read is retried a few times.
pub(super) fn read_report_descriptor(state: &State) -> Option<alloc::vec::Vec<u8>> {
    let desc = &state.descriptor;
    let rd_len = u16::from_le_bytes([desc[4], desc[5]]) as usize;
    let rd_reg = u16::from_le_bytes([desc[6], desc[7]]);
    if !(4..=REPORT_DESC_MAX).contains(&rd_len) {
        return None;
    }
    let mut buf = alloc::vec![0u8; rd_len];
    for attempt in 0..REPORT_DESC_ATTEMPTS {
        if attempt > 0 {
            let _ = nonos_libc::mk_idle_ms(REPORT_DESC_RETRY_MS);
        }
        if write_read(state.i2c_port, state.addr, &rd_reg.to_le_bytes(), &mut buf) == Some(rd_len) {
            return Some(buf);
        }
    }
    None
}
