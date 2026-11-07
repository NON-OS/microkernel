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

//! Times the admission phases of a capsule launch on the host.
//!
//! nonos-launch-bench [REPO_ROOT]
//!
//! Prints one table per phase: what was timed, how many inputs, and the
//! median and p90 over the repeats. All numbers are labelled `host`.

mod capabilities;
mod crypto;
mod security;
mod smp;
mod syscall;
mod time;

mod admission;
mod audit;
mod clock;
mod gate;
mod image;
mod inputs;
mod ring;
mod stark;
mod stats;

use std::path::PathBuf;

fn main() {
    let root = std::env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let inputs = match inputs::load(&root) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("nonos-launch-bench: {e}");
            std::process::exit(2);
        }
    };
    println!("# nonos-launch-bench (label: host)");
    println!("# capsules: {}  repeats: {}", inputs.capsules.len(), stats::REPEATS);
    admission::run(&inputs);
    stark::run(&root, &inputs.capsules);
    image::run(&root);
    let gate_ns = gate::run();
    clock::run(gate_ns);
    ring::run();
    audit::run();
}
