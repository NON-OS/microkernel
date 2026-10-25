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

//! Host proofs of capsule_shield_vectors (Test 5). Its modules are mounted
//! by #[path], unchanged; only machine.rs, which asks the NONOS kernel, is
//! stood in for (machine.rs here), and its main.rs stays target-only. The
//! tests (src/tests) prove nothing. This binary is the full proving run
//! (full.rs), which the test lane does not run: `cargo test` builds it and
//! runs its tests, never its main.

#[path = "../../capsule_shield_vectors/src/cache.rs"]
mod cache;
#[path = "../../capsule_shield_vectors/src/pinned.rs"]
mod pinned;
#[path = "../../capsule_shield_vectors/src/pool.rs"]
mod pool;
#[path = "../../capsule_shield_vectors/src/prove.rs"]
mod prove;
#[path = "../../capsule_shield_vectors/src/run.rs"]
mod run;
#[path = "../../capsule_shield_vectors/src/serial.rs"]
mod serial;
#[path = "../../capsule_shield_vectors/src/vectors.rs"]
mod vectors;

// heap.rs declares `mod counted;`, which lives under heap/. A file mounted by
// #[path] looks for its modules beside itself, so heap.rs is reached through
// an inline module naming the capsule's src/, where `mod heap;` looks as the
// capsule's own main.rs does.
#[path = "../../capsule_shield_vectors/src"]
mod capsule {
    pub mod heap;
}
use capsule::heap;

mod full;
mod machine;

#[cfg(test)]
mod tests;

fn main() {
    std::process::exit(full::run());
}
