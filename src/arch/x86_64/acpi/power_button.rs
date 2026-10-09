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
//! lock an interrupt must not wait on.

use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering};

use super::hw::gas::gas_read;
use super::hw::port_bus::PortBus;
use super::hw::power_button::{acpi_mode, classify, clear_value, fresh, pressed, Button};
use super::parser;
use crate::arch::x86_64::port::{inw, outw};
use crate::kernel_core::surface_registry::{try_post_input, InputEvent};

/// The Power key's code (KEYCODE_POWER in the keyboard drivers and
/// app_skeleton), and the key event kinds (nonos_libc INPUT_KIND_KEY_*).
const KEYCODE_POWER: u32 = 0x1304;
const KEY_DOWN: u16 = 0;
const KEY_UP: u16 = 1;
/// The paced tick runs at 100 Hz.
const MS_PER_TICK: u64 = 10;

static STS_A: AtomicU16 = AtomicU16::new(0);
static STS_B: AtomicU16 = AtomicU16::new(0);
/// A press read from the hardware and not yet in the input ring.
static PENDING: AtomicBool = AtomicBool::new(false);
static LAST_PRESS_MS: AtomicU64 = AtomicU64::new(u64::MAX);

fn say(line: &[u8]) {
    crate::sys::serial::println(line);
}

/// Decide once, after the ACPI tables are parsed, whether the button can be
/// polled, and say in one line what the power button will do.
pub fn init() {
    let Some(fadt) = parser::with_data(|d| d.fadt).flatten() else {
        say(b"[ACPI] power button: no FADT, the button stays with the firmware");
        return;
    };
    match classify(&fadt) {
        Button::Fixed { sts_a, sts_b } => {
            let cnt = gas_read(&mut PortBus, &fadt.pm1a_cnt).unwrap_or(0) as u16;
            if !acpi_mode(cnt) {
                say(b"[ACPI] power button: the firmware left ACPI mode off (SCI_EN clear), so the button stays with the firmware");
                return;
            }
            // A press from before boot is not a request to shut down now.
            // SAFETY: the port is the FADT's PM1 status register, which is
            // write-one-to-clear; only the power button bit is written.
            unsafe {
                outw(sts_a, clear_value());
                if sts_b != 0 {
                    outw(sts_b, clear_value());
                }
            }
            STS_B.store(sts_b, Ordering::Relaxed);
            STS_A.store(sts_a, Ordering::Release);
            say(b"[ACPI] power button: fixed feature, polled; a press goes to the desktop as the Power key");
        }
        Button::ControlMethod => say(b"[ACPI] power button: reported through AML (PNP0C0C), which NONOS cannot run; use the Shut Down menu, or hold the button to force power off"),
        Button::HwReduced => say(b"[ACPI] power button: hardware-reduced platform, reported through AML, which NONOS cannot run; use the Shut Down menu"),
        Button::Unreachable => say(b"[ACPI] power button: its status register is not an I/O port NONOS reads; use the Shut Down menu"),
    }
}

/// One look at the button, from the boot CPU's paced tick.
pub fn poll(ticks: u64) {
    let sts_a = STS_A.load(Ordering::Acquire);
    if sts_a == 0 {
        return;
    }
    let sts_b = STS_B.load(Ordering::Relaxed);
    let now_ms = ticks.saturating_mul(MS_PER_TICK);
    // SAFETY: the ports are the FADT's PM1 status registers, checked to be
    // 16-bit System I/O at init; the bit is write-one-to-clear and only it
    // is written.
    let hit = unsafe {
        let a = pressed(inw(sts_a));
        let b = sts_b != 0 && pressed(inw(sts_b));
        if a {
            outw(sts_a, clear_value());
        }
        if b {
            outw(sts_b, clear_value());
        }
        a || b
    };
    if hit && accept(now_ms) {
        PENDING.store(true, Ordering::Release);
    }
    if PENDING.load(Ordering::Acquire) && deliver() {
        PENDING.store(false, Ordering::Release);
    }
}

fn accept(now_ms: u64) -> bool {
    let last = LAST_PRESS_MS.load(Ordering::Relaxed);
    if !fresh((last != u64::MAX).then_some(last), now_ms) {
        return false;
    }
    LAST_PRESS_MS.store(now_ms, Ordering::Relaxed);
    true
}

/// The press and its release into the input ring. False when the ring was
/// busy or full, to try again next tick.
fn deliver() -> bool {
    let key = |kind| InputEvent {
        kind,
        flags: 0,
        code: KEYCODE_POWER,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    if !matches!(try_post_input(key(KEY_DOWN)), Some(Ok(()))) {
        return false;
    }
    // The shell acts on the press. The release is for whoever tracks held
    // keys, and a ring still busy after a few tries loses only that.
    for _ in 0..4 {
        if matches!(try_post_input(key(KEY_UP)), Some(Ok(()))) {
            break;
        }
    }
    true
}
