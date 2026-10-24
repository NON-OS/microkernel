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

//! What stands in for the kernel around the mounted files. Each source can be
//! turned off for the calling thread: the seeded RNG (as if not yet seeded),
//! the hardware entropy source, and the CPU's random instruction. A working
//! source answers with a count, so a test can see which source answered.

use std::cell::Cell;
use std::sync::Mutex;

thread_local! {
    pub(crate) static SEEDED: Cell<bool> = const { Cell::new(true) };
    pub(crate) static ENTROPY: Cell<bool> = const { Cell::new(true) };
    pub(crate) static CPU: Cell<bool> = const { Cell::new(true) };
    static COUNT: Cell<u64> = const { Cell::new(0) };
}

/// Sets which sources answer on this thread: the seeded RNG, the hardware
/// entropy source, and the CPU's random instruction.
pub fn sources(seeded: bool, entropy: bool, cpu: bool) {
    SEEDED.set(seeded);
    ENTROPY.set(entropy);
    CPU.set(cpu);
}

/// The value each source gives: its tag in the top byte, and below it a
/// count spread over every bit, so no two values share their low bits.
pub(crate) fn next(tag: u64) -> u64 {
    let n = COUNT.get() + 1;
    COUNT.set(n);
    (tag << 56) | (n.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 8)
}

pub const SEEDED_TAG: u64 = 0x5e;
pub const ENTROPY_TAG: u64 = 0xe7;
pub const CPU_TAG: u64 = 0xc9;

pub static SERIAL: Mutex<Vec<u8>> = Mutex::new(Vec::new());

pub mod arch {
    pub fn read_time_counter() -> u64 {
        42
    }
    pub mod cpu_random {
        pub fn random_u64() -> Option<u64> {
            crate::stand_in_rng::cpu_random64()
        }
    }
}

pub mod sys {
    pub mod serial {
        pub fn print(s: &[u8]) {
            super::super::SERIAL.lock().unwrap().extend_from_slice(s);
        }
    }
}
