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

//! Test 5, on the machine: the four pinned production wallet vectors proved
//! by the nox_prover nonos.shield links, built as nonos.shield is built
//! (the std layer, its heap, a rayon worker per online core), each proof
//! and its format 7 form held to the pinned bytes. One serial line per
//! check, PASS or FAIL, then a DONE line with the count. Development
//! images only.
//!
//! The first vector proves from nothing, as a wallet with no cache would,
//! and the tree it builds must be the cache nonos.shield ships. The other
//! three prove from the shipped cache, as nonos.shield does.

mod cache;
mod heap;
mod machine;
mod pinned;
mod pool;
mod prove;
mod run;
mod serial;
mod vectors;

use cache::SHIPPED;
use serial::{line, TAG};

fn main() {
    let mut out = std::io::stdout();
    pool::start();
    println!(
        "{TAG} start: {} cores online, {} prover workers, {} MiB memory, {} MiB free",
        machine::cpus_online(),
        pool::workers(),
        machine::mem_total_mib(),
        machine::mem_free_mib()
    );
    if pool::workers() == 0 {
        line(&mut out, false, "no thread could be started for the prover");
        println!("{TAG} DONE 0 of {} vectors", vectors::ALL.len());
        return;
    }
    let cache_ok = cache::shipped_cache(&mut out);
    let mut passed = 0u32;
    let mut cache: Option<Vec<u8>> = None;
    for (i, v) in vectors::ALL.iter().enumerate() {
        // The first proves from nothing; the rest from the shipped cache,
        // or from the tree the first built when this build ships none.
        if i == 1 && !SHIPPED.is_empty() {
            cache = Some(SHIPPED.to_vec());
        }
        cache = prove::prove(&mut out, v, cache, &mut passed);
    }
    println!(
        "{TAG} DONE {passed} of {} vectors byte for byte{}",
        vectors::ALL.len(),
        if cache_ok { "" } else { ", shipped cache FAILED" }
    );
}
