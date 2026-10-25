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

//! Arming the poll, once device interrupt routing is up: the SCI's IO-APIC line
//! masked, then PWRBTN_EN set (`hw::power_button_arm` has the rule and why).
//! Task context, never the tick: the IO-APIC calls take locks an interrupt
//! must not wait on.

use core::sync::atomic::{AtomicBool, Ordering};

use super::super::hw::power_button_arm::{arm as plan, ArmStep};
use super::{say, EN_A, EN_B, STS_A};
use crate::arch::x86_64::interrupt::ioapic;
use crate::arch::x86_64::port::{inw, outw};
use crate::sys::serial::Line;

static ARMED: AtomicBool = AtomicBool::new(false);

/// Arm the poll for the SCI on legacy IRQ `sci_irq`, unless a capsule holds
/// it (`held`). A line that cannot be masked leaves PWRBTN_EN as it was.
pub fn arm(sci_irq: u8, held: bool) {
    let (en_a, en_b) = (EN_A.load(Ordering::Relaxed), EN_B.load(Ordering::Relaxed));
    if STS_A.load(Ordering::Acquire) == 0 || en_a == 0 || ARMED.swap(true, Ordering::AcqRel) {
        return;
    }
    // SAFETY: both ports are the FADT's PM1 enable registers, checked at init
    // to be 16-bit System I/O; a read has no side effect. A and B read ORed
    // and are written alike (ACPI 6.5, 4.8.1.1).
    let now = unsafe { inw(en_a) | if en_b != 0 { inw(en_b) } else { 0 } };
    let Some(steps) = plan(held, now) else {
        say(b"[ACPI] power button: a capsule holds the SCI and sets its own enables");
        return;
    };
    for step in steps {
        match step {
            ArmStep::MaskSci => {
                if !ioapic::is_initialized() {
                    say(b"[ACPI] power button: device interrupt routing is not up, so PWRBTN_EN stays clear");
                    return;
                }
                if ioapic::mask(ioapic::gsi_for_irq(u32::from(sci_irq)), true).is_err() {
                    say(b"[ACPI] power button: the SCI line could not be masked, so PWRBTN_EN stays clear");
                    return;
                }
            }
            // SAFETY: as above; only the enable registers are written, with
            // the value the rule gives: what they held plus PWRBTN_EN.
            ArmStep::Enable(v) => unsafe {
                outw(en_a, v);
                if en_b != 0 {
                    outw(en_b, v);
                }
            },
        }
    }
    // SAFETY: as above.
    let back = unsafe { inw(en_a) };
    Line::new().str(b"[ACPI] power button: SCI line masked, PM1_EN=").hex(u64::from(back)).end();
}
