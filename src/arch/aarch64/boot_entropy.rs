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

//! The seed a loader hands the kernel in the device tree's `/chosen`
//! `rng-seed` (Devicetree Specification v0.4, 3.6): on a board whose RNG the
//! kernel does not drive, such as a Raspberry Pi 4's Cortex-A72 with no RNDR,
//! the one source of entropy there is before userland.
//!
//! Kept at parse time, before the heap, and condensed and wiped when the RNG
//! takes it. The blob keeps no copy: boot zeroes the property once read.

use spin::Mutex;

/// Fewer bytes than a seed's 256 bits are not counted as a seed.
pub const MIN_SEED: usize = 32;
const MAX_SEED: usize = 64;
const CONTEXT: &str = "NONOS 2026-10 device tree rng-seed v1";

struct Seed {
    bytes: [u8; MAX_SEED],
    len: usize,
}

static SEED: Mutex<Option<Seed>> = Mutex::new(None);

/// Keep `seed` if it is long enough to count; boot calls this once.
pub fn keep(seed: &[u8]) {
    if seed.len() < MIN_SEED {
        return;
    }
    let len = seed.len().min(MAX_SEED);
    let mut bytes = [0u8; MAX_SEED];
    bytes[..len].copy_from_slice(&seed[..len]);
    *SEED.lock() = Some(Seed { bytes, len });
}

/// The seed condensed to 32 bytes, once; the kept copy is wiped.
pub fn take() -> Option<[u8; 32]> {
    let mut slot = SEED.lock();
    let seed = slot.as_mut()?;
    let mut out = [0u8; 32];
    crate::crypto::hash::blake3_derive_key(CONTEXT, &seed.bytes[..seed.len], &mut out);
    for b in seed.bytes.iter_mut() {
        // SAFETY: `b` is a live, exclusively borrowed byte of the kept seed.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    *slot = None;
    Some(out)
}

/// A key for `context` from the kept seed, without using the seed up: the
/// RNG still takes it. BLAKE3's derive mode keeps each context's output
/// independent of every other's and of the seed itself.
pub fn derive(context: &str) -> Option<[u8; 32]> {
    let slot = SEED.lock();
    let seed = slot.as_ref()?;
    let mut out = [0u8; 32];
    crate::crypto::hash::blake3_derive_key(context, &seed.bytes[..seed.len], &mut out);
    Some(out)
}
