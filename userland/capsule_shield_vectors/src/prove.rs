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

//! One vector proved, timed and measured, then held to the pinned bytes.

use std::io::Write;
use std::time::Instant;

use nox_prover::ENTROPY_BYTES;

use crate::serial::{line, TAG};
use crate::vectors::{self, Vector};
use crate::{cache, heap, machine, pinned, run};

/// Prove `v`, from `cache` when held, and hold the result to the pinned
/// bytes. Returns the cache the proof had or built.
pub(crate) fn prove(
    out: &mut impl Write,
    v: &Vector,
    cache: Option<Vec<u8>>,
    passed: &mut u32,
) -> Option<Vec<u8>> {
    let Some(entropy) = vectors::entropy(v) else {
        line(out, false, &format!("{}: the entropy file is not {ENTROPY_BYTES} bytes", v.name));
        return cache;
    };
    let free_before = machine::mem_free_mib();
    heap::reset_peak();
    let watch = machine::Watch::start();
    let start = Instant::now();
    let proved = run::run(v, &entropy, cache.as_deref(), start);
    let ms = start.elapsed().as_millis();
    let least_free = watch.stop();
    let (proof, built) = match proved {
        Ok(done) => done,
        Err(why) => {
            line(out, false, &format!("{}: refused after {ms} ms: {why}", v.name));
            return cache;
        }
    };
    let from_nothing = cache.is_none();
    let from = if from_nothing { "from nothing" } else { "from the periodic cache" };
    println!(
        "{TAG} {}: proved {from} in {ms} ms, heap peak {} MiB, machine free {} MiB before, least {} MiB",
        v.name,
        heap::peak_mib(),
        free_before,
        least_free
    );
    let cache = cache.or(built);
    let Some(top) = cache.as_deref() else {
        line(out, false, &format!("{}: no periodic cache after the proof", v.name));
        return None;
    };
    if from_nothing {
        cache::built_is_shipped(out, v.name, top);
    }
    if pinned::held(out, v, &proof, top) {
        *passed += 1;
    }
    cache
}
