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

use crate::protocol::{Request, HDR_LEN, STATUS_LEN};
use crate::scan::Scanner;
use crate::server::respond;
use crate::state::State;

/// The counters, then where the search for the device stands
/// (scan/report.rs), then the re-bind generation: devices served after the
/// first, and media served afresh (scan/asked.rs), as a u32. It is raised
/// whenever what is served may be another medium, so the kernel's client
/// (src/hardware/usb_msc_capsule) can tell and read what it trusts of the
/// disk again before it writes; one that reads only the counters and the
/// report reads this reply as before.
pub fn handle(state: &State, scanner: &Scanner, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let out = &mut tx[HDR_LEN + STATUS_LEN..];
    let n = state.write_snapshot(out);
    let n = n + scanner.report.encode(&mut out[n..]);
    out[n..n + 4].copy_from_slice(&scanner.rebinds.to_le_bytes());
    let _ = respond::payload(sender_pid, req, n + 4, tx);
}
