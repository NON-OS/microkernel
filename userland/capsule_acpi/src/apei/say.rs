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

//! One log line per error record: where it came from, its severity, the
//! section's type and the error's type.

use crate::sys::log::Line;
use nonos_aml::apei::{record, Entry, Record};

pub fn severity(s: u32) -> &'static [u8] {
    match s {
        0 => b"recoverable",
        1 => b"fatal",
        2 => b"corrected",
        _ => b"informational",
    }
}

/// Source, severity, section type and error type: never an address, a
/// device's position or a record's raw bytes.
pub fn say(prefix: &[u8], e: &Entry) {
    let l = Line::new(prefix).str(severity(e.severity)).str(b": ");
    match record(&e.section, &e.data) {
        Record::Processor { error_type, level } => {
            let l = l.str(b"processor, type ").hex(u64::from(error_type));
            l.str(b" level ").dec(u64::from(level)).end()
        }
        Record::Memory { kind: Some(k) } => l.str(b"memory, type ").dec(u64::from(k)).end(),
        Record::Memory { kind: None } => l.str(b"memory, type not given").end(),
        Record::Pcie { uncorrectable: Some(u) } => {
            l.str(b"pcie, uncorrectable ").hex(u64::from(u)).end()
        }
        Record::Pcie { uncorrectable: None } => l.str(b"pcie, status not given").end(),
        Record::Other => l.str(b"section of an unread type").end(),
    }
}
