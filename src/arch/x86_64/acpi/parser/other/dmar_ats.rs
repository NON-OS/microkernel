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

//! DMAR's ATS structures, read. An ATSR names root ports whose devices
//! may use Address Translation Services; a SATC names SoC devices with an
//! address translation cache, flag bit 0 when they need ATS to work. The
//! kernel enables ATS nowhere: every context entry takes untranslated
//! requests only, so a device's own translations are refused. These counts
//! let the boot log say what was left off.

use core::sync::atomic::{AtomicUsize, Ordering};

pub(super) const ATSR_TYPE: u16 = 2;
pub(super) const SATC_TYPE: u16 = 5;
const SATC_ATC_REQUIRED: u8 = 1 << 0;

static ATSR: AtomicUsize = AtomicUsize::new(0);
static SATC: AtomicUsize = AtomicUsize::new(0);
static SATC_REQUIRED: AtomicUsize = AtomicUsize::new(0);

/// `bytes` is one ATSR or SATC structure, at least its 8-byte head.
pub(super) fn record(kind: u16, bytes: &[u8]) {
    match kind {
        ATSR_TYPE => ATSR.fetch_add(1, Ordering::Relaxed),
        _ if bytes[4] & SATC_ATC_REQUIRED != 0 => SATC_REQUIRED.fetch_add(1, Ordering::Relaxed),
        _ => SATC.fetch_add(1, Ordering::Relaxed),
    };
}

/// ATSR structures, SATCs, and SATCs whose devices need ATS.
pub fn ats_structures() -> (usize, usize, usize) {
    let load = |c: &AtomicUsize| c.load(Ordering::Relaxed);
    let required = load(&SATC_REQUIRED);
    (load(&ATSR), load(&SATC) + required, required)
}
