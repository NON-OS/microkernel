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

use super::super::types::Pid;
use super::types::ProcessTable;

impl ProcessTable {
    /// Pids whose alarm has expired, for the timer interrupt. Never waits and
    /// never allocates: a writer busy on another cpu costs this tick nothing,
    /// the alarm is caught on the next, and a heap lock held by the code this
    /// interrupt broke into is never touched.
    pub fn expired_alarms(&self, out: &mut [Pid]) -> usize {
        let Some(table) = self.inner.try_read() else {
            return 0;
        };
        let mut n = 0;
        for pcb in table.iter() {
            if n == out.len() {
                break;
            }
            if pcb.check_alarm_expired() {
                out[n] = pcb.pid;
                n += 1;
            }
        }
        n
    }
}
