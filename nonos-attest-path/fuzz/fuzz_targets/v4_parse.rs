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

//! Any bytes, any expected kind, any proof bound: `parse_v4` never panics,
//! and a trailer it accepts has exactly one encoding. Re-encoding what it
//! read gives back the input byte for byte, so no two byte strings can be
//! read as the same trailer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_attest_path::{encode_v4, parse_v4, Kind, MAGIC, MAX_PROOF_V4};

const KINDS: [Kind; 3] = [Kind::Kernel, Kind::Capsule, Kind::Bootloader];

fuzz_target!(|data: &[u8]| {
    let Some((&sel, rest)) = data.split_first() else {
        return;
    };
    let Some((&bound, t)) = rest.split_first() else {
        return;
    };
    let expect = KINDS[usize::from(sel) % KINDS.len()];
    /* Small bounds, the real gates' bounds and the edges all come up. */
    let max_proof = match bound {
        0 => 0,
        1 => MAX_PROOF_V4,
        2 => MAX_PROOF_V4 + 1,
        b => usize::from(b) * 61,
    };
    let Some(read) = parse_v4(t, expect, max_proof) else {
        return;
    };
    assert!(read.kind == expect);
    assert!(!read.proof.is_empty() && read.proof.len() <= max_proof);
    assert!(max_proof <= MAX_PROOF_V4);
    assert!(read.path.starts_with(&MAGIC));
    let again = encode_v4(read.kind, read.path, read.proof);
    assert!(again.as_deref() == Some(t), "accepted bytes are not the canonical encoding");
});
