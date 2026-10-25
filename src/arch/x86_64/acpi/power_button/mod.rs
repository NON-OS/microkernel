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

//! The power button, polled.
//!
//! With no AML interpreter there is no SCI handler, so a press of the
//! fixed-feature button is found by reading PWRBTN_STS from the boot CPU's
//! paced tick every 100 ms (`hw::power_button` says why that works and when
//! it does not). A press clears the bit and becomes a Power key press and
//! release in the input ring, which the input router sends to the desktop
//! shell. The kernel does not power off on its own: whether the desktop may
//! shut the machine down is the decision 0.9.2 leaves open on capsule_power,
//! and the shell says so when the key arrives. Holding the button four
//! seconds still forces the machine off in hardware.
//!
//! Everything the tick touches is an atomic or an I/O port: the ports are
//! read from the FADT once at init, because the parser's data sits behind a
//! lock an interrupt must not wait on. `arm` runs once, when device interrupt
//! routing is up, and is the one step that takes a lock: it sets PWRBTN_EN, which
//! QEMU needs before it latches a press (`hw::power_button_arm`).

mod arm;
mod deliver;
mod gate;
mod init;
mod poll;

use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU64};

pub use arm::arm;
pub use init::init;
pub use poll::poll;

static STS_A: AtomicU16 = AtomicU16::new(0);
static STS_B: AtomicU16 = AtomicU16::new(0);
/// The PM1 enable registers, 0 where the block has none NONOS can reach.
static EN_A: AtomicU16 = AtomicU16::new(0);
static EN_B: AtomicU16 = AtomicU16::new(0);
/// A press read from the hardware and not yet in the input ring.
static PENDING: AtomicBool = AtomicBool::new(false);
static LAST_PRESS_MS: AtomicU64 = AtomicU64::new(u64::MAX);

fn say(line: &[u8]) {
    crate::sys::serial::println(line);
}
