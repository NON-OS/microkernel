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

use super::boot_report::BootReport;
use super::key_changes::key_changes;
use super::types::Keyboard;

impl Keyboard {
    /// Take one boot report from the interrupt endpoint.
    pub fn feed(&mut self, raw: &[u8]) {
        if let Some(report) = BootReport::parse(raw) {
            self.apply(report);
        }
    }

    /// Take one report, read in the boot layout or through a report
    /// descriptor. The modifiers are set first so every key event carries
    /// this report's.
    pub fn apply(&mut self, report: BootReport) {
        self.modifiers = report.modifiers;
        let held = self.prev;
        self.prev = key_changes(&held, &report, |key, pressed| self.push_key(key, pressed));
    }
}
