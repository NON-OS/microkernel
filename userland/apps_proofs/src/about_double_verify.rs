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

//! The kernel's table, registry and boot record, each a fixed answer.

#[path = "../../capsule_about/src/about/data/verify/types.rs"]
mod types;

use alloc::vec::Vec;
pub use types::{Census, Check, Live, Recorded, Verdict};

pub struct Attested {
    pub pid: u32,
    pub measurement: [u8; 32],
    pub authority: u8,
    pub name: [u8; 16],
    pub name_len: usize,
}

#[derive(Clone, Copy)]
pub struct Admitted {
    pub measurement: [u8; 32],
    pub root: [u8; 32],
    pub epoch: u64,
}

pub struct Loader {
    pub verdict: Verdict,
    pub claim: &'static [u8],
    pub admitted: Option<Admitted>,
    pub code: Option<u32>,
}

pub fn attested() -> Option<Vec<Attested>> {
    let (name, name_len, authority) = (*b"capsule_about\0\0\0", 13, 0);
    let one = |i: u32| Attested { pid: i, measurement: [i as u8; 32], authority, name, name_len };
    Some((10..16).map(one).collect())
}

pub fn recorded() -> Option<Recorded> {
    let (kernel_signature, secure_boot, attestation, proof) =
        (Verdict::Holds, Verdict::Broken, Verdict::Holds, Verdict::Unknown);
    let (kernel_hash, program_hash) = ([1; 32], [2; 32]);
    Some(Recorded { kernel_signature, secure_boot, attestation, proof, kernel_hash, program_hash })
}

pub fn loader() -> Loader {
    let admitted = Some(Admitted { measurement: [3; 32], root: [4; 32], epoch: 3 });
    Loader { verdict: Verdict::Holds, claim: b"bootloader measured", admitted, code: None }
}

pub fn live() -> Option<Live> {
    let check = |claim, verdict, found, of| Check { claim, verdict, found, of };
    let checks = [
        check(&b"no capsule reaches raw hardware"[..], Verdict::Holds, 0, 44),
        check(b"every capsule was admitted", Verdict::Broken, 2, 44),
        check(b"nothing runs unrecorded", Verdict::Unknown, 0, 0),
        check(b"this window holds no FileSystem", Verdict::Holds, 1, 1),
    ];
    let census = Census { capsules: 44, filesystem: 3, raw_hardware: 9, own_mask: 0x8000_1819 };
    Some(Live { checks, census })
}
