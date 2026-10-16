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

//! The longest request by its parts, summed from the driver's own limits
//! and held below the kernel's wait as the proofs build: the served card
//! identified again in the request's look, then the longest write. Each
//! wait is taken to its limit and the parked millisecond past it, on the
//! RTS522A, which has the most registers.

use crate::card::limits::*;
use crate::hw::{ACCESS_MS, PHY_MS};
use crate::serve::slot::SETTLE_MS;

/// The kernel's wait for a reply (src/services/lifecycle/reply_wait.rs).
pub const KERNEL_WAIT_MS: u64 = 35_000;

const fn past(ms: u64) -> u64 {
    ms + 1
}
const REG: u64 = past(ACCESS_MS);
const CMD: u64 = past(COMMAND_MS);
const REGS: u64 = past(REGS_MS);

/// The slot powered down (card/power_off.rs): two buffers, four registers.
const POWER_OFF: u64 = 2 * REGS + 4 * REG;

/// HAIMR's registers on the way (registers_tests.rs counts them): sixteen,
/// and the stop and clear after the CMD8 a version 1.x card does not answer.
pub const REGISTERS: u64 = 16 + 3;

/// A card brought up (card/up.rs) to served. Buffers: six of registers
/// (four powering the slot, timing, the 3.3 V drive), the SSC clock at
/// 400 kHz and 25 MHz. Commands: CMD0 and CMD8; ACMD41's window, and the
/// CMD55 and ACMD41 that run past it; CMD2, 3, 9, 16, CMD55, ACMD6 and
/// CMD7's busy. Then the PHY write's busy wait, the registers, and the
/// sleeps on the way (100, 20 and 1 ms powering, 10 and 10 after, 1 per
/// clock, 1 after CMD0, 10 before that last ACMD41).
const BUFFERS: u64 = 6 * REGS + 2 * past(SSC_MS);
const COMMANDS: u64 = 2 * CMD + POWER_UP_MS + 2 * CMD + 6 * CMD + past(BUSY_MS);
const BRING_UP: u64 = BUFFERS + COMMANDS + past(PHY_MS) + REGISTERS * REG + 154;

/// The look that finds the served card in doubt (serve/slot.rs,
/// serve/change.rs): the settle, the slot down, the card up again.
const IDENTIFY: u64 = SETTLE_MS + POWER_OFF + BRING_UP;

/// CMD25, the data, CMD12's busy, the programming wait and the CMD13 that
/// found the card still busy, the same whether the data phase failed or
/// not (card/finish.rs); and the engines stopped and the error cleared
/// after each of the three that can fail at their limit.
const WRITE: u64 = CMD + past(DATA_MS) + past(BUSY_MS) + past(PROGRAM_MS) + CMD + 9 * REG;

/// CMD24, the data, and the programming wait a failed one still gets, with
/// the stop and clear after the data and after the last CMD13.
pub const SINGLE_WRITE: u64 = CMD + past(DATA_MS) + past(PROGRAM_MS) + CMD + 6 * REG;

// Held when the proofs build: a limit raised past the kernel's wait fails
// the build, not a run.
const _: () = assert!(
    IDENTIFY + WRITE < KERNEL_WAIT_MS,
    "the longest write after the longest identification outlasts the kernel's wait"
);
const _: () = assert!(SINGLE_WRITE < WRITE, "a single-block write outlasts a multi-block one");
