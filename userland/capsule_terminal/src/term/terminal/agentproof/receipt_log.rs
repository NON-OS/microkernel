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

//! A headless run's receipt as the serial log keeps it, for the Gauntlet to
//! read back (tools/agentproof_gauntlet/receipt.py):
//!
//! ```text
//! [AGENTPROOF-RECEIPT] v=1 bytes=<n> seal=<16 hex>
//! [AGENTPROOF-RECEIPT] <the receipt's next 96 bytes, as hex>
//! [AGENTPROOF-RECEIPT] v=1 end
//! ```

use alloc::string::String;

use nonos_libc::mk_debug;

/// Receipt bytes a row carries: a row is one write the kernel takes whole.
const ROW: usize = 96;

/// The receipt, a head line, its rows as hex, and an end line; false if
/// any of them was not written.
pub fn keep(napr: &[u8], seal: u64) -> bool {
    let head = alloc::format!("[AGENTPROOF-RECEIPT] v=1 bytes={} seal={seal:016x}\n", napr.len());
    let rows = napr.chunks(ROW).map(|r| {
        let hex: String = r.iter().map(|b| alloc::format!("{b:02x}")).collect();
        alloc::format!("[AGENTPROOF-RECEIPT] {hex}\n")
    });
    let end = String::from("[AGENTPROOF-RECEIPT] v=1 end\n");
    [head].into_iter().chain(rows).chain([end]).all(|l| mk_debug(l.as_ptr(), l.len()) >= 0)
}
