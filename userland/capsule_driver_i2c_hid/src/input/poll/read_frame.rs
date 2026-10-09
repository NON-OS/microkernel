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

use super::signal_raw_report::signal_raw_report;
use crate::i2c_client::write_read;
use crate::state::{State, FRAME_MAX};

/// Read one frame from the input register into `buf`. The length read, or
/// None when the read failed or returned less than the length prefix.
pub(super) fn read_frame(state: &mut State, buf: &mut [u8; FRAME_MAX]) -> Option<usize> {
    state.last_read_had_report = false;
    // Read only as much as the device declares (with a floor for firmwares
    // that under-report wMaxInputLength). The read length is transfer TIME on
    // the wire: a 256-byte read at fast-mode speed takes longer than the
    // pad's report period, so it straddles a device-side report update and
    // tears the coordinates mid-transfer. Keeping the read within the real
    // report size keeps it inside one report interval.
    let declared = state.input_len;
    let len = if declared >= 5 { declared.min(buf.len()) } else { 64 };
    state.input_polls = state.input_polls.wrapping_add(1);
    // Spec first: after reset the device auto-points at the input register
    // and reports come from a bare read. Devices that only answer a
    // register-addressed read get the fallback.
    let n = match write_read(state.i2c_port, state.addr, &[], &mut buf[..len]) {
        Some(n) if n >= 2 => n,
        _ => {
            let reg = state.input_register.to_le_bytes();
            write_read(state.i2c_port, state.addr, &reg, &mut buf[..len])?
        }
    };
    if n < 2 {
        return None;
    }
    state.input_reports = state.input_reports.wrapping_add(1);
    state.last_read_had_report = buf[0] != 0 || buf[1] != 0;
    // A 0x0000 length prefix is the "nothing pending" / reset sentinel, not a
    // report; only count frames that carry data.
    if buf[0] != 0 || buf[1] != 0 {
        signal_raw_report();
        // The first few raw frames go to the boot console so a photograph
        // shows the exact wire bytes next to the parsed layout.
        if state.frame_dumps < 6 {
            state.frame_dumps += 1;
            let tag = alloc::format!("[i2chid] frm{} n={}:", state.frame_dumps, n);
            crate::diag::hex_line(&tag, &buf[..n.min(16)]);
        }
    }
    Some(n)
}
