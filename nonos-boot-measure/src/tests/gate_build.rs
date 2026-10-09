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

//! A real enrollment: `fixture/loader.efi`, a PE laid out as `pe_build` lays
//! one out, enrolled by `nonos-stark-enroll bootloader` into `fixture/root.bin`
//! and `fixture/trailer.bin`, a v4 trailer with a real STARK proof. A boot that
//! started it, as the firmware would log it, and a record over its root.

use super::boot_log::boot;
use super::log_build::{ev, extend, EV_APP};
use super::record_build::record;
use crate::authenticode::digest;

pub(super) const LOADER: &[u8] = include_bytes!("fixture/loader.efi");
pub(super) const TRAILER: &[u8] = include_bytes!("fixture/trailer.bin");

pub(super) fn root() -> [u8; 32] {
    *include_bytes!("fixture/root.bin")
}

pub(super) fn measurement() -> [u8; 32] {
    digest(LOADER).expect("the fixture is a PE")
}

/// The log of a boot that started the fixture, then the PCR 4 applications in
/// `after`, and the PCR 4 the TPM would hold, extended here by hand.
pub(super) fn booted(after: &[[u8; 32]]) -> (Vec<u8>, [u8; 32]) {
    let (mut log, mut pcr) = boot(&[ev(4, EV_APP, measurement())]);
    pcr = extend(pcr, measurement());
    for &d in after {
        log.extend(ev(4, EV_APP, d));
        pcr = extend(pcr, d);
    }
    (log, pcr)
}

/// A record over the fixture's root at `epoch`, signed by the test key.
pub(super) fn signed(epoch: u64) -> Vec<u8> {
    record(root(), epoch)
}
