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

//! The kernel's random source. Keys, nonces and addresses come from here.
//! If it fails there is nothing safe to pair or hide behind, so the
//! capsule stops rather than go on with values an observer could guess.

use super::env::CapsuleEnv;
use super::say::say;
use nonos_bt_bond::LocalIrk;
use nonos_bt_smp::pairing::Random;
use nonos_libc::{crypto_random, mk_exit};

const EXIT_NO_RANDOM: i32 = 3;

pub fn draw(out: &mut [u8]) {
    if crypto_random(out.as_mut_ptr(), out.len()) < 0 {
        say(&[b"the kernel's random source failed; stopping"]);
        mk_exit(EXIT_NO_RANDOM);
    }
}

impl Random for CapsuleEnv {
    fn fill(&mut self, out: &mut [u8]) {
        draw(out);
    }
}

pub fn draw16() -> [u8; 16] {
    let mut k = [0u8; 16];
    draw(&mut k);
    k
}

/// This host's IRK for this run, when the store could not keep one: peers
/// bonded now cannot resolve this host after the capsule restarts.
pub fn fresh_irk() -> LocalIrk {
    say(&[b"this host's IRK could not be kept; it lasts until the capsule stops"]);
    // LocalIrk refuses only all-zero keys, which a working source does not
    // give twice running.
    for _ in 0..2 {
        if let Ok(irk) = LocalIrk::new(draw16()) {
            return irk;
        }
    }
    say(&[b"the kernel's random source gave zeros; stopping"]);
    mk_exit(EXIT_NO_RANDOM)
}
