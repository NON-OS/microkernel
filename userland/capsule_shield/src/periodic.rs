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

//! The periodic cache the prover proves from, shipped in this capsule.
//!
//! The Nix build makes it with nox_bench at the STARKs commit the flake pins
//! and checks it there (tools/nix/periodic.nix); build.rs embeds it. The
//! phones' core reads the cache from the wallet's own folder, where the apps
//! keep it, so the shipped file is written there when the folder holds any
//! other bytes. Without it each proof costs about a quarter more time and
//! half a gigabyte more memory, building the tree the file already holds.
//!
//! Nothing is offered to the prover unless nox_prover::load_cache accepts
//! it, and load_cache refuses a file whose root is not PERIODIC_ROOT. A
//! capsule built without a cache ships none, and the first proof builds the
//! cache and keeps it, as on a phone.

use std::path::Path;

use nox_shield_core::prover::launch::cache;

static SHIPPED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/periodic.top"));

/// True when the prover takes `bytes` as its periodic cache.
fn accepted(bytes: &[u8]) -> bool {
    nox_prover::load_cache(bytes).is_ok()
}

/// Put the shipped cache in the wallet folder `dir`, unless it is there.
///
/// A held file that differs from the shipped one is replaced: the tree is a
/// constant of the circuit, so any other bytes are damaged or from another
/// circuit, and load_cache checks the root only, not the levels below it.
pub fn seed(dir: &Path) {
    if SHIPPED.is_empty() || !accepted(SHIPPED) {
        return;
    }
    if cache::read(dir).as_deref() != Some(SHIPPED) {
        cache::keep(dir, SHIPPED);
    }
}

#[cfg(test)]
#[path = "periodic_test.rs"]
mod periodic_test;
