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

//! S5 read ahead, then entered from what was read.
//!
//! The panic path reads it before the heap wipe, through `try_with_data`, so
//! the entry after takes no lock; with the tables' lock held it reads nothing
//! and the machine is wiped and halted instead of waiting.

use super::aml::sleep_obj::SleepPackage;
use super::data::AcpiData;
use super::error::{AcpiError, AcpiResult};
use super::hw::fadt_decode::FadtInfo;
use super::hw::port_bus::{busy_wait_us, PortBus};
use super::hw::sleep::{enter_sleep, SleepError, SleepTypes};
use crate::hardware::broker::acpi::s5_choice::choose;

type Read = (Option<FadtInfo>, Option<(u8, u8)>, Option<(u8, u8)>);
type With = fn(fn(&AcpiData) -> Read) -> Option<Read>;

/// What S5 needs: the decoded FADT and the SLP_TYP pair.
#[derive(Clone, Copy)]
pub struct PreparedOff {
    fadt: FadtInfo,
    types: SleepTypes,
}

/// Read what S5 needs through `parser::with_data` or `try_with_data`.
pub fn prepare_shutdown(with: With) -> AcpiResult<PreparedOff> {
    fn read(d: &AcpiData) -> Read {
        let pair = |p: Option<SleepPackage>| p.map(|p| (p.slp_typ_a, p.slp_typ_b));
        (d.fadt, pair(d.s5), pair(d.s5_interp))
    }
    let (fadt, scan, interp) = with(read).ok_or(AcpiError::NotInitialized)?;
    let fadt = fadt.ok_or(AcpiError::FadtNotFound)?;
    let ((a, b), _) = choose(scan, interp).ok_or(AcpiError::PowerStateNotSupported)?;
    Ok(PreparedOff { fadt, types: SleepTypes { a, b } })
}

fn wbinvd() {
    // SAFETY: WBINVD has no memory operands and is legal at CPL 0, where this runs.
    unsafe { core::arch::asm!("wbinvd", options(nostack, preserves_flags)) };
}

/// Enter S5 from what `prepare_shutdown` read: port writes and a wait,
/// no lock, no allocation and no log.
pub fn shutdown_prepared(off: &PreparedOff) -> AcpiResult<()> {
    // Nothing past this point expects to be interrupted.
    crate::arch::x86_64::interrupt::cli();

    let mut bus = PortBus;
    for _ in 0..2 {
        match enter_sleep(&mut bus, &off.fadt, off.types, wbinvd) {
            Ok(()) => {}
            Err(SleepError::NoControlRegister) => return Err(AcpiError::PowerStateNotSupported),
            Err(SleepError::WriteFailed) => return Err(AcpiError::HardwareAccessFailed),
        }
        // Power should be gone within milliseconds; give slow boards 3 s.
        busy_wait_us(3_000_000);
    }
    Err(AcpiError::HardwareAccessFailed)
}
