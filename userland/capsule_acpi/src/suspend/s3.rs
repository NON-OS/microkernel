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

//! PREPARE_SLEEP for S3 (state 3) and suspend-to-idle (state 0), reached
//! only with the switch on (`switch.rs`). S3 runs the firmware's sequence
//! with the policy's wake devices and undoes it when the kernel's half
//! answers NotWired; suspend-to-idle calls the Low Power S0 Idle device's
//! entry and exit notifications around an idle the kernel does not have
//! yet, and logs how far LPIT's residency counters advanced between them.
//! Each logs what it did.

use super::hardware::Unwired;
use super::lpit::{note_residency, states};
use super::switch::WAKE_DEVICES;
use crate::boot::{evaluator, Acpi};
use crate::server::protocol::{EIO, ENOENT};
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::namespace::ROOT;
use nonos_aml::platform::resolve_text;
use nonos_aml::suspend::{find_lps0, idle_enter, idle_exit, suspend_s3, SuspendError};

pub fn s3(acpi: &mut Acpi) -> i32 {
    let ns = &acpi.aml.ns;
    let wake: Vec<_> = WAKE_DEVICES.iter().filter_map(|p| resolve_text(ns, ROOT, p)).collect();
    let mut hw = Unwired { lock_held: acpi.host.lock_held };
    let Acpi { aml, host, .. } = acpi;
    let out = suspend_s3(aml, &mut evaluator(host), &mut hw, &wake, 0);
    let l = Line::new(b"S3: armed ").dec(out.armed.len() as u64);
    let l = l.str(b", passed over ").dec(out.skipped.len() as u64);
    let why: &[u8] = match out.error {
        None => b"; slept and woke",
        Some(SuspendError::NoS3) => b"; no usable \\_S3, nothing run",
        Some(SuspendError::LockHeld) => b"; the Global Lock was held: undone",
        Some(SuspendError::Hardware(_)) => b"; the kernel's half is not wired: undone",
    };
    l.str(why).end();
    match (out.entered, out.error) {
        (true, _) => 0,
        (_, Some(SuspendError::NoS3)) => ENOENT,
        _ => EIO,
    }
}

pub fn idle(acpi: &mut Acpi) -> i32 {
    let Acpi { aml, host, devices, .. } = acpi;
    let mut eval = evaluator(host);
    let Some(l) = find_lps0(aml, &mut eval, devices) else { return ENOENT };
    let before = states();
    let on = idle_enter(aml, &mut eval, &l).len() as u64;
    let off = idle_exit(aml, &mut eval, &l).len() as u64;
    note_residency(&before, &states());
    let line = Line::new(b"s2idle: ").dec(on).str(b" entry and ").dec(off);
    line.str(b" exit notifications; the kernel's idle is not wired").end();
    EIO
}
