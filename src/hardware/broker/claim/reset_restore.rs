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

//! Putting back what an FLR cleared (K10a), and the reset's pause.

use super::reset_function::Function;

const BUS_MASTER: u16 = 1 << 2;
pub(super) const INITIATE_FLR: u16 = 1 << 15;

impl Function {
    /// BARs, ROM, cache line and interrupt line, then Device Control, then
    /// the command register last, Bus Master still clear.
    pub(super) fn restore_header(&mut self) {
        for i in [3usize, 4, 5, 6, 7, 8, 9, 12, 15] {
            let _ = self.cfg.write32(i as u16 * 4, self.header[i]);
        }
        if let Some(p) = self.pcie {
            let _ = self.cfg.write16(p + 8, self.control.0 & !INITIATE_FLR);
            let _ = self.cfg.write16(p + 0x28, self.control.1);
        }
        let _ = self.cfg.set_command(self.header[1] as u16 & !BUS_MASTER);
    }
}

/// Wait `ms` milliseconds: asleep on the sleeper's process where reset_wait
/// allows it, spinning otherwise.
pub(super) fn pause_ms(ms: u64, sleeper: Option<u32>) {
    let Some(pid) = super::reset_wait::sleeper(sleeper, crate::process::current_pid()) else {
        let end = crate::time::monotonic_ns().saturating_add(ms.saturating_mul(1_000_000));
        while crate::time::monotonic_ns() < end {
            core::hint::spin_loop();
        }
        return;
    };
    let end = crate::time::timestamp_millis().saturating_add(ms);
    while crate::time::timestamp_millis() < end {
        crate::sched::sleep_until(pid, end);
        crate::sched::yield_now();
    }
}
