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

//! Any bytes as a capsule manifest: the kernel's decoder never panics, and it
//! accepts exactly what the host signer's decoder accepts. A byte string one
//! side reads and the other refuses is a capsule manifest the two disagree on.

#![no_main]

use admission_proofs::capsule_manifest::decode::decode;
use libfuzzer_sys::fuzz_target;
use nonos_capsule_sign::verify::decode::decode_manifest;

fuzz_target!(|data: &[u8]| {
    let kernel = decode(data).is_ok();
    let signer = decode_manifest(data).is_ok();
    assert!(kernel == signer, "kernel accepts: {kernel}, signer accepts: {signer}");
});
