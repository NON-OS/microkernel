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

//! What the ACPI capsule armed, disarmed when it leaves (F-107): with its
//! SCI handler gone, an enabled GPE or fixed event raises an SCI nobody
//! serves. Every GPE enable (ACPI 6.5, 4.8.4.1: a block's second half) and
//! both PM1 enables (4.8.3.1) are cleared; status bits are left for the
//! next capsule, and the power button's latches unenabled, so the poll
//! still sees a press. And the Global Lock (5.2.10.1) it said it held
//! released. Pure over a RegisterBus: `kernel_proofs` mounts it.

use super::fadt_decode::FadtInfo;
use super::gas::{gas_read, gas_write, Gas, RegisterBus};
use super::sleep::{pm1_halves, PM1_CNT_GBL_RLS, PM1_CNT_SLP_EN};

/// The Global Lock word's pending and owned bits (5.2.10.1, recalled).
pub const PENDING: u32 = 1;
pub const OWNED: u32 = 2;

fn enables_off<B: RegisterBus>(bus: &mut B, block: &Gas, len: u8) {
    if !block.is_present() || len < 2 {
        return;
    }
    let regs = u64::from(len / 2);
    for i in 0..regs {
        let _ = gas_write(bus, &block.at_offset(regs + i, 8), 0);
    }
}

/// Every event enable the capsule could have set, cleared. A
/// hardware-reduced platform has neither block (4.1): its events are
/// interrupts, which go back with the capsule's grants.
pub fn disarm<B: RegisterBus>(bus: &mut B, fadt: &FadtInfo) {
    if fadt.is_hw_reduced() {
        return;
    }
    for evt in [&fadt.pm1a_evt, &fadt.pm1b_evt] {
        let (_, enable) = pm1_halves(evt);
        if enable.is_present() {
            let _ = gas_write(bus, &enable, 0);
        }
    }
    enables_off(bus, &fadt.gpe0, fadt.gpe0_len);
    enables_off(bus, &fadt.gpe1, fadt.gpe1_len);
}

/// The word to store to release a lock found as `old`, and whether
/// firmware set pending and waits for GBL_RLS.
pub fn release_word(old: u32) -> (u32, bool) {
    (old & !(PENDING | OWNED), old & PENDING != 0)
}

/// GBL_RLS to PM1 control: the lock is free. The other bits go back as
/// read, but never SLP_EN, so nothing is put to sleep.
pub fn global_release<B: RegisterBus>(bus: &mut B, fadt: &FadtInfo) {
    for cnt in [&fadt.pm1a_cnt, &fadt.pm1b_cnt] {
        if cnt.is_present() {
            let v = gas_read(bus, cnt).unwrap_or(0) as u16 & !PM1_CNT_SLP_EN;
            let _ = gas_write(bus, cnt, u64::from(v | PM1_CNT_GBL_RLS));
        }
    }
}
