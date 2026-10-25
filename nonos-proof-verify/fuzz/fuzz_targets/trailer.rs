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

//! The whole check against a real enrollment (the test fixture's capsule A,
//! a real STARK) admits only the honest trailer, and the slot reader never
//! panics. Most inputs flip, extend or cut the honest v4 trailer or the
//! honest development path, so the search stays near the bytes that pass.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_attest_path::{capsule_context, Kind};
use nonos_proof_verify::{blake3_of, check_v4, opens, Proof};

const ROOT: [u8; 32] = *include_bytes!("../../tests/fixture/caps_root.bin");
const DEV_ROOT: [u8; 32] = *include_bytes!("../../tests/fixture/dev_root.bin");
const HONEST: &[u8] = include_bytes!("../../tests/fixture/a.trailer");
const HONEST_DEV: &[u8] = include_bytes!("../../tests/fixture/a.dev");
const IMAGE: &[u8] = include_bytes!("../../tests/fixture/a.elf");

fuzz_target!(|data: &[u8]| {
    let ctx = capsule_context(&blake3_of(IMAGE), 0x18, 1);
    let Some((&mode, rest)) = data.split_first() else {
        return;
    };
    let base = if mode & 8 == 0 { HONEST } else { HONEST_DEV };
    let t: Vec<u8> = match mode & 3 {
        0 => rest.to_vec(),
        1 => {
            // XOR the input over the honest bytes from an offset it chooses.
            let mut t = base.to_vec();
            let (at, flip) = rest.split_at(rest.len().min(4));
            let mut off = [0u8; 4];
            off[..at.len()].copy_from_slice(at);
            let start = u32::from_le_bytes(off) as usize % t.len();
            for (x, b) in t[start..].iter_mut().zip(flip) {
                *x ^= b;
            }
            t
        }
        2 => {
            let mut t = base.to_vec();
            t.extend_from_slice(rest);
            t
        }
        _ => {
            let cut = rest.first().map_or(1, |&b| usize::from(b) + 1);
            base[..base.len().saturating_sub(cut)].to_vec()
        }
    };
    let _ = opens(&t);
    for allow_dev in [false, true] {
        let root = if t.starts_with(b"NZKPATH1") { DEV_ROOT } else { ROOT };
        match check_v4(&root, Kind::Capsule, &ctx, &t, allow_dev) {
            Ok(Proof::Stark) => assert!(t == HONEST, "a trailer other than the honest one passed"),
            Ok(Proof::PathOnly) => {
                assert!(allow_dev && t == HONEST_DEV, "a development path passed unasked or altered")
            }
            Err(r) => {
                let _ = (r.stage(), r.code(), r.reason());
            }
        }
    }
});
