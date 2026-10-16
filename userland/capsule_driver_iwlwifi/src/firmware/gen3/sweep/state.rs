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

//! A sweep's state and counts.

/// One background sweep and its running counts.
#[derive(Default)]
pub struct Sweep {
    pub(super) started_ms: Option<u64>,
    pub(super) budget_ms: u64,
    /// Management frames handed up, across all sweeps.
    pub frames: u32,
    /// Sweeps the firmware completed.
    pub completed: u32,
    /// Sweeps given up with no completion.
    pub stalled: u32,
    /// The uid of the sweep in flight (or the last one), and the next one's.
    pub(super) uid: u32,
    pub(super) next_uid: u32,
    /// A sweep given up as stalled and not yet aborted: its uid.
    pub(super) stalled_uid: Option<u32>,
}

impl Sweep {
    pub fn new() -> Self {
        Self::default()
    }

    /// A sweep is in flight.
    pub fn running(&self) -> bool {
        self.started_ms.is_some()
    }
}
