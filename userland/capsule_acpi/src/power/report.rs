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

//! Sending the word the kernel keeps (`word`) when it changed or a minute
//! has passed.

use super::say::say;
use super::state::Power;
use super::word::word;
use crate::sys::acpi_power::power_report;

pub fn send(p: &mut Power) {
    let w = word(p);
    if p.reported != Some(w) {
        say(w);
    }
    if power_report(w).is_ok() {
        p.reported = Some(w);
    }
}
