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

//! A driver brought up on a bench, and the broker calls it left behind.

use crate::bench::Bench;
use crate::model::Svga as Model;
use crate::pool::Pool;
use crate::setup::{run, Driver};

pub fn up(bench: &mut Bench, tune: impl FnOnce(&mut Model)) -> Driver<Pool> {
    run(bench.with(tune), Pool::default()).expect("bring-up on a well-behaved adapter")
}

/// The releases a failed attempt made, in order.
pub fn released() -> Vec<(&'static str, u64)> {
    released_since(0)
}

/// The releases made after the first `n` broker calls on this thread.
pub fn released_since(n: usize) -> Vec<(&'static str, u64)> {
    nonos_libc::calls()
        .into_iter()
        .skip(n)
        .filter(|(c, _)| c.ends_with("unmap") || c.ends_with("release"))
        .collect()
}

/// The firmware mode the tests start from: what UEFI's GOP leaves set.
pub fn firmware(s: &mut Model) {
    (s.enable, s.width, s.height, s.bpp, s.config_done) = (1, 1280, 800, 32, 1);
}
