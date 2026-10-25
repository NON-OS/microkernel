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

//! The TCG log: a log that replays is within both caps, and the kernel's
//! own replay (nonos-boot-measure, compiled from its source), which
//! accepts every log this one does, gives the same PCR 4 and the same
//! last application.

#![no_main]

#[allow(dead_code, unused_imports, clippy::all)]
#[path = "../../../../nonos-boot-measure/src/tcg/mod.rs"]
mod kernel_tcg;

use libfuzzer_sys::fuzz_target;
use nsh_attest::consts::MAX_LOG;
use nsh_attest::log::{replay, MAX_EVENTS};

fuzz_target!(|data: &[u8]| {
    if let Ok(r) = replay(data) {
        assert!(data.len() <= MAX_LOG && r.events <= MAX_EVENTS);
        let k = kernel_tcg::replay(data).expect("the kernel reads every log this one does");
        assert_eq!(r.pcrs[4], k.pcr4);
        assert_eq!(r.last_application, k.last_application);
        assert_eq!(r.events, k.events);
    }
});
