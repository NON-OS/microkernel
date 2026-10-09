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

//! This boot's two slots, from what the loader left: the loader as the
//! kernel's check of it admitted it, with the loader's own trailer; and this
//! kernel's file, which the loader hashed and held to its STARK before the
//! jump. The kernel's slot is worked out once; the file does not change.

use nonos_attest_path::Kind;
use spin::Once;

use super::footer::regions;
use super::record::{encode, RECORD_LEN};
use super::slot::{slot, Slot};
use crate::boot::handoff::types::MODULE_KIND_BOOT_TRAILER;
use crate::security::boot::loader_check::{verdict, Verdict};
use crate::security::boot::modules::module_bytes;
use crate::syscall::microkernel::install_source::KIND_KERNEL_IMAGE;

/// Longer than any v4 trailer, whose proof is bounded by `MAX_PROOF_V4`.
const MAX_TRAILER: u64 = 4 * 1024 * 1024;
/// Longer than any signed kernel file this loader boots.
const MAX_KERNEL_FILE: u64 = 256 * 1024 * 1024;

static KERNEL: Once<Option<Slot>> = Once::new();

/// The record, or `None` when the loader was not admitted by the kernel's
/// check or either image carries no trailer of its kind.
pub fn boot_slots_record() -> Option<[u8; RECORD_LEN]> {
    let loader = loader_slot()?;
    let kernel = (*KERNEL.call_once(kernel_slot))?;
    Some(encode(&loader, &kernel))
}

fn loader_slot() -> Option<Slot> {
    let admitted = match verdict()? {
        Verdict::Measured(a) | Verdict::SelfReported(a) => a,
        Verdict::Refused(_) | Verdict::NoEvidence => return None,
    };
    let trailer = module_bytes(MODULE_KIND_BOOT_TRAILER, MAX_TRAILER)?;
    let s = slot(Kind::Bootloader, &admitted.measurement, trailer)?;
    /* The root the check admitted the loader under, or no slot at all. */
    (s.root == admitted.root).then_some(s)
}

fn kernel_slot() -> Option<Slot> {
    let file = module_bytes(KIND_KERNEL_IMAGE, MAX_KERNEL_FILE)?;
    let r = regions(file)?;
    slot(Kind::Kernel, blake3::hash(r.kernel).as_bytes(), r.proof)
}
