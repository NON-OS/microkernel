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

//! `HOST_PROOF`: one that parses signs all but its signature, holds whole
//! registry entries, and carries an enrolment of a well-formed host.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_wire::keys::HYBRID_SIG_LEN;
use nsh_wire::names;
use nsh_wire::{HostProof, ENTRY_LEN};

fuzz_target!(|data: &[u8]| {
    if let Ok(p) = HostProof::parse(data) {
        assert_eq!(p.signed, &data[..data.len() - HYBRID_SIG_LEN]);
        assert_eq!(p.entries.len() % ENTRY_LEN, 0);
        assert_eq!(p.entries().count() * ENTRY_LEN, p.entries.len());
        assert!(!p.doc.is_empty());
        assert!(names::host(p.enrol.host).is_ok());
        assert!(p.enrol.fleet_id.iter().any(|b| *b != 0));
    }
});
