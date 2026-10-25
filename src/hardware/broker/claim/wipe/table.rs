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

//! The wipe records (L78), one per function at most, beside the claims and
//! taken under them (claims, then wipes). A record is the kernel's, never a
//! thread's, so no kill can strand it; `busy` marks one a step is acting on
//! with no lock held, so two steppers never write the same function.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

use super::caps::ResetCaps;
use super::holder_rule::Name;
use super::marker_rule::Marker;
use super::method_rule::Method;
use super::phase_rule::Phase;
use crate::drivers::pci::types::PciAddress;

/// What a reset clears that the function's next holder needs back.
#[derive(Clone, Copy, Debug)]
pub(super) struct Saved {
    pub(super) command: u16,
    pub(super) devctl: Option<u16>,
    pub(super) devctl2: Option<u16>,
    pub(super) cls: u8,
    pub(super) lt: u8,
    pub(super) int_line: u8,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Wipe {
    pub(super) device_id: u64,
    pub(super) epoch: u64,
    pub(super) by: Option<Name>,
    pub(super) phase: Phase,
    pub(super) method: Option<Method>,
    pub(super) caps: ResetCaps,
    pub(super) address: PciAddress,
    pub(super) marker: Option<Marker>,
    pub(super) saved: Option<Saved>,
    pub(super) bme_off: bool,
    pub(super) busy: bool,
}

pub(super) static WIPES: Mutex<Vec<Wipe>> = Mutex::new(Vec::new());
/// Records in a phase a step moves on, so a step with none costs a load.
pub(super) static PENDING: AtomicUsize = AtomicUsize::new(0);
