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

//! The whole-image hashes a spawn takes. Before this change there were two: the
//! manifest's payload hash in the kernel's own BLAKE3, and the attestation
//! measurement in the blake3 crate built `pure`. Now the measurement is
//! taken once (`capsule_attest::measure`) and both checks compare against
//! it. Timed on the in-tree BusyBox guest and on buffers of the sizes
//! catalogue capsules reach.

use std::fs;
use std::path::Path;

use crate::crypto::hash::blake3::Hasher;
use crate::stats::{row, time, Spread};

fn sum(a: &Spread, b: &Spread) -> Spread {
    Spread { median_ns: a.median_ns + b.median_ns, p90_ns: a.p90_ns + b.p90_ns }
}

fn kernel_blake3(bytes: &[u8]) -> [u8; 32] {
    let mut h = Hasher::new();
    crate::smp::in_serve_units(bytes, |piece| {
        h.update(piece);
    });
    h.finalize()
}

fn crate_blake3(bytes: &[u8]) -> [u8; 32] {
    crate::security::capsule_attest::measure::measure(bytes)
}

fn both(what: &str, bytes: &[u8]) {
    if kernel_blake3(bytes) != crate_blake3(bytes) {
        println!("{what}: the two BLAKE3 implementations disagree");
        return;
    }
    let mib = bytes.len() as f64 / (1024.0 * 1024.0);
    let k = time(|| {
        std::hint::black_box(kernel_blake3(bytes));
    });
    row(&format!("before: payload hash, kernel BLAKE3 {what} {mib:.2} MiB"), 1, &k);
    let c = time(|| {
        std::hint::black_box(crate_blake3(bytes));
    });
    row(&format!("measure, blake3 pure {what} {mib:.2} MiB"), 1, &c);
    row(&format!("image hashing per spawn, before {what} {mib:.2} MiB"), 1, &sum(&k, &c));
    row(&format!("image hashing per spawn, after {what} {mib:.2} MiB"), 1, &c);
}

pub fn run(root: &Path) {
    let busybox = root.join("userland/capsule_linux/guests/busybox.elf");
    match fs::read(&busybox) {
        Ok(b) => both("busybox.elf", &b),
        Err(e) => println!("{}: {e}", busybox.display()),
    }
    for mib in [1usize, 4, 16] {
        let buf: Vec<u8> = (0..mib << 20).map(|i| (i as u32).wrapping_mul(2_654_435_761) as u8).collect();
        both("synthetic", &buf);
    }
}
