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

//! Refusal variants for the test-only boot profile. Each starts from an honest
//! enrollment and is changed afterwards, so a refusal at boot is the gate's
//! doing and not a proof enrollment never made. Each is checked here first:
//! the honest trailer must pass and every variant must be refused.

mod side;

use nonos_attest_path::{encode_v4, parse_v4, Kind, MAX_PROOF_V4};

use crate::commands::spec;
use crate::context::{capsule_context, kernel_context, POLICY_EPOCH};
use crate::io::{die, hash_of, read, read_root, write};
use crate::stark::check_v4;

/// `refusal-variants <root.bin> <CAPS:image:trailer> <outdir>`.
pub fn run(root_path: &str, spec_s: &str, outdir: &str) {
    let root = read_root(root_path);
    let (caps, image, trailer) = spec(spec_s);
    let h = hash_of(image);
    let ctx = capsule_context(&h, caps);
    let honest = read(trailer);
    let gate = |t: &[u8]| check_v4(&root, Kind::Capsule, &ctx, t);
    if let Err(e) = gate(&honest) {
        die(&format!("the honest trailer is refused, nothing to vary: {e}"));
    }
    let v = parse_v4(&honest, Kind::Capsule, MAX_PROOF_V4).unwrap_or_else(|| die("honest parse"));
    let mut proof = v.proof.to_vec();
    let at = proof.len() / 2;
    proof[at] ^= 0x01;
    let flip = encode_v4(Kind::Capsule, v.path, &proof).unwrap_or_else(|| die("flip encode"));

    let kernel_kind = side::enroll_one(Kind::Kernel, &kernel_context(&h));
    let stale_ctx = nonos_attest_path::capsule_context(&h, caps, POLICY_EPOCH - 1);
    let stale = side::enroll_one(Kind::Capsule, &stale_ctx);

    for (name, t) in [("flip", &flip), ("kernel_kind", &kernel_kind), ("stale_epoch", &stale)] {
        match gate(t) {
            Ok(()) => die(&format!("variant {name} was admitted by the gate's own check")),
            Err(e) => println!("  {name}: refused, {e}"),
        }
        write(&format!("{outdir}/{name}.trailer"), t);
    }
}
