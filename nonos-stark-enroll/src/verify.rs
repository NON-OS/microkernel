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

//! Re-checks of shipped artifacts as the gates check them: the path and the STARK.

use nonos_attest_path::Kind;

use crate::commands::spec;
use crate::context::{capsule_context, kernel_context};
use crate::io::{die, hash_of, hex, read, read_root};
use crate::measure::authenticode_of;
use crate::stark::check_v4;

/// Re-check a shipped capsule set with the spawn gate's own check.
pub fn verify_capsules(root_path: &str, specs: &[String]) {
    let root = read_root(root_path);
    let mut failed = 0usize;
    for s in specs {
        let (caps, image, trailer) = spec(s);
        let ctx = capsule_context(&hash_of(image), caps);
        let ok = check_v4(&root, Kind::Capsule, &ctx, &read(trailer)).is_ok();
        println!("  {}  {image}", if ok { "ok  " } else { "FAIL" });
        failed += usize::from(!ok);
    }
    if failed > 0 {
        die(&format!("{failed} of {} capsule paths failed under root {}", specs.len(), hex(&root)));
    }
    println!("verified {} capsule paths under root {}", specs.len(), hex(&root));
}

/// Re-check the kernel's trailer, the check the bootloader repeats before the jump.
pub fn verify_kernel(root_path: &str, image: &str, trailer: &str) {
    let root = read_root(root_path);
    if let Err(e) = check_v4(&root, Kind::Kernel, &kernel_context(&hash_of(image)), &read(trailer))
    {
        die(&format!("kernel self-attestation FAILED under root {}: {e}", hex(&root)));
    }
    println!("verified kernel self-attestation under root {}", hex(&root));
}

/// Re-check the bootloader's trailer under its own tree, with the bootloader leaf kind,
/// so a kernel leaf can never stand in for it.
pub fn verify_bootloader(root_path: &str, image: &str, trailer: &str) {
    let root = read_root(root_path);
    let ctx = kernel_context(&authenticode_of(image));
    if let Err(e) = check_v4(&root, Kind::Bootloader, &ctx, &read(trailer)) {
        die(&format!("bootloader attestation FAILED under root {}: {e}", hex(&root)));
    }
    println!("verified bootloader attestation under root {}", hex(&root));
}
