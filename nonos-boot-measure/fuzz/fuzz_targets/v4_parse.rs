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

//! The v4 reader on any bytes, for every kind and any bound: no panic, a
//! trailer it accepts re-encodes to itself, and its parts lie inside it.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_attest_path::{encode_v4, parse_v4, Kind, MAX_PROOF_V4};

fuzz_target!(|data: &[u8]| {
    let Some((&b, t)) = data.split_first() else { return };
    let bound = 1 + (usize::from(b) << 10).min(MAX_PROOF_V4 - 1);
    for kind in [Kind::Kernel, Kind::Capsule, Kind::Bootloader, Kind::Pad] {
        for max in [bound, MAX_PROOF_V4, MAX_PROOF_V4 + 1, 0] {
            let Some(v) = parse_v4(t, kind, max) else { continue };
            assert!(kind != Kind::Pad && v.kind == kind && max <= MAX_PROOF_V4);
            assert!(!v.proof.is_empty() && v.proof.len() <= max);
            assert!(v.path.len() + v.proof.len() + 17 == t.len());
            assert_eq!(encode_v4(kind, v.path, v.proof).as_deref(), Some(t));
        }
    }
});
