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

//! One log line per error record: its severity, where it came from, and
//! what the section says.

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

pub fn say(prefix: &[u8], e: &Entry) {
    let l = Line::new(prefix).str(severity(e.severity)).str(b": ");
    match record(&e.section, &e.data) {
        Record::Processor { error_type, level, processor } => {
            let l = l
                .str(b"processor type ")
                .hex(u64::from(error_type))
                .str(b" level ")
                .dec(u64::from(level));
            match processor {
                Some(p) => l.str(b" cpu ").hex(p).end(),
                None => l.end(),
            }
        }
        Record::Memory { address, node, card, module, bank, kind } => {
            let l = match address {
                Some(a) => l.str(b"memory at ").hex(a),
                None => l.str(b"memory"),
            };
            let l = l.str(b" node ").dec(u64::from(node)).str(b" card ").dec(u64::from(card));
            let l = l.str(b" module ").dec(u64::from(module)).str(b" bank ").dec(u64::from(bank));
            l.str(b" type ").dec(u64::from(kind)).end()
        }
        Record::Pcie { segment, bus, device, function, vendor, id, uncorrectable } => {
            let l = l.str(b"pcie ").hex(u64::from(segment)).str(b":").hex(u64::from(bus)).str(b":");
            let l = l.hex(u64::from(device)).str(b".").dec(u64::from(function)).str(b" ");
            let l = l.hex(u64::from(vendor)).str(b":").hex(u64::from(id));
            match uncorrectable {
                Some(u) => l.str(b" uncorrectable ").hex(u64::from(u)).end(),
                None => l.end(),
            }
        }
        Record::Other => l.str(b"section of an unread type").end(),
    }
}
