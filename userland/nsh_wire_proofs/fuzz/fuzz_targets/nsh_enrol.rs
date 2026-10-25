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

//! An enrolment certificate: one that parses names a well-formed host and
//! a fleet, is no larger than the largest certificate, and signs all but
//! its signature, its boot applications among what is signed: whole
//! digests, at most eight, none zero and none twice.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_wire::keys::HYBRID_SIG_LEN;
use nsh_wire::{names, Enrol, BOOT_APP_LEN, MAX_BOOT_APPS, MAX_ENROL};

fuzz_target!(|data: &[u8]| {
    if let Ok(e) = Enrol::parse(data) {
        assert!(names::host(e.host).is_ok());
        assert!(e.fleet_id.iter().any(|b| *b != 0));
        assert!(data.len() <= MAX_ENROL);
        assert_eq!(e.signed, &data[..data.len() - HYBRID_SIG_LEN]);
        let apps: Vec<_> = e.boot_applications().collect();
        assert_eq!(e.boot_apps.len(), apps.len() * BOOT_APP_LEN);
        assert!(apps.len() <= MAX_BOOT_APPS);
        for (i, a) in apps.iter().enumerate() {
            assert!(a.iter().any(|b| *b != 0) && !apps[..i].contains(a));
        }
        let at = e.boot_apps.as_ptr() as usize - e.signed.as_ptr() as usize;
        assert!(at + e.boot_apps.len() <= e.signed.len(), "the section is signed");
    }
});
