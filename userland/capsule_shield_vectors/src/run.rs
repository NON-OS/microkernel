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

//! The prover's call, from the periodic cache when held, each phase on the serial line.

use std::time::Instant;

use nox_prover::{Options, Phase, Proof};

use crate::serial::TAG;
use crate::vectors::Vector;

/// The proof of `v`, and the cache it built when it had none.
pub(crate) fn run(
    v: &Vector,
    entropy: &[u8],
    cache: Option<&[u8]>,
    start: Instant,
) -> Result<(Proof, Option<Vec<u8>>), String> {
    let report = |p: Phase, f: f32| {
        println!(
            "{TAG} {}: {p:?} done, {:.0}% at {} ms",
            v.name,
            f * 100.0,
            start.elapsed().as_millis()
        );
    };
    match cache {
        Some(top) => {
            let opts = Options { cache: Some(top), progress: Some(&report), cancel: None };
            nox_prover::prove_with(v.request, v.seed, entropy, &opts)
                .map(|(p, _)| (p, None))
                .map_err(|e| e.to_string())
        }
        None => {
            nox_prover::prove_keeping_cache(v.request, v.seed, entropy).map(|(p, c)| (p, Some(c)))
        }
    }
}
