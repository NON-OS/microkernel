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

use super::control::lfence;
use super::control_intr::pause;
use super::frequency_pit_io::{inb, outb};
use super::tsc::rdtsc;
use crate::arch::x86_64::time::tsc::calibration::math::reference_timeout_ticks;
use crate::arch::x86_64::time::tsc::constants::MAX_FREQUENCY;

const PIT_FREQUENCY: u64 = 1_193_182;
const CALIBRATE_MS: u64 = 50;
const PIT_CHANNEL_0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;
pub(super) const FALLBACK_HZ: u64 = 2_400_000_000;

pub fn calibrate_tsc_with_pit() -> u64 {
    let pit_count = (PIT_FREQUENCY * CALIBRATE_MS) / 1000;
    unsafe {
        outb(PIT_COMMAND, 0x30);
        outb(PIT_CHANNEL_0, (pit_count & 0xFF) as u8);
        outb(PIT_CHANNEL_0, ((pit_count >> 8) & 0xFF) as u8);
        lfence();
        let tsc_start = rdtsc();
        // A gated PIT never raises OUT0, and this wait had no bound. The TSC
        // always counts, so it ends once twice the window has passed at the
        // fastest TSC accepted, as the boot CPU's PIT measurement does.
        let timeout = reference_timeout_ticks(CALIBRATE_MS, MAX_FREQUENCY);
        loop {
            outb(PIT_COMMAND, 0xE2);
            let status = inb(PIT_CHANNEL_0);
            if (status & 0x80) != 0 {
                break;
            }
            if rdtsc().wrapping_sub(tsc_start) > timeout {
                return FALLBACK_HZ;
            }
            pause();
        }
        lfence();
        let tsc_end = rdtsc();
        let elapsed = tsc_end.saturating_sub(tsc_start);
        let freq = (elapsed * 1000) / CALIBRATE_MS;
        if freq >= 500_000_000 && freq <= 6_000_000_000 {
            freq
        } else {
            FALLBACK_HZ
        }
    }
}
