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

//! How far a carried answer has got, said every couple of megabytes.

use alloc::format;

use super::CloneJob;
use crate::command::output::Output;

/// How often a transfer says how much has come.
const REPORT_EVERY: usize = 2 * 1024 * 1024;

impl CloneJob {
    /// A line every couple of megabytes while a large answer arrives.
    pub(super) fn report(&mut self, out: &mut Output<'_>) {
        let Some((_, exchange)) = self.carrying.as_ref() else { return };
        let got = exchange.received();
        if got < self.reported.saturating_add(REPORT_EVERY) {
            return;
        }
        self.reported = got;
        let line = format!("git clone: received {} MiB", got / (1024 * 1024));
        out.writeln(line.as_bytes());
    }
}
