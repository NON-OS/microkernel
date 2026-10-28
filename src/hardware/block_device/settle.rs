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

//! The end of a pass that found no boot disk. Another NONOS disk is kept
//! once every disk answered, or once the wait for a refused one runs out,
//! unless the loader read the store from the disk it named: then no other
//! disk is kept (`other_disk`), the loader's copy answers store reads, and
//! the boot keeps nothing. A later pass that finds the boot disk keeps it.

use core::sync::atomic::{AtomicBool, Ordering};

use super::backend::Backend;
use super::booted;
use super::gpt::Places;
use super::kept::{keep, none};
use super::other_disk::{other_disk, Other};
use super::wait::{caller_gone, waited_for_boot_disk};
use super::BlockDeviceError;

static TOLD_ALONE: AtomicBool = AtomicBool::new(false);

pub(super) fn settle(
    other: Option<(Backend, Places)>,
    refused: Option<BlockDeviceError>,
) -> Result<Backend, BlockDeviceError> {
    let Some((b, p)) = other else { return Err(none()) };
    // Never for a caller killed meanwhile: its asks failed for it (L81).
    let waited = refused.is_some() && !caller_gone() && waited_for_boot_disk();
    let (named, own_copy) = (booted::partition().is_some(), booted::store_from_boot_disk());
    match (other_disk(named, own_copy, refused.is_some(), waited), refused) {
        (Other::Keep, _) => Ok(keep(b, p)),
        (Other::Wait, Some(e)) => Err(e),
        (Other::Wait | Other::Leave, _) => Err(left_alone()),
    }
}

/* Said once: a dead disk to every caller, so the data volume is held in RAM. */
fn left_alone() -> BlockDeviceError {
    if !TOLD_ALONE.swap(true, Ordering::Relaxed) {
        let line = "[BLOCK] the disk this boot came from is not driven; no other NONOS disk is kept, so this boot keeps nothing and reads the loader's copy of the store";
        crate::sys::serial::println(line.as_bytes());
        crate::log::warn!("{}", line);
    }
    BlockDeviceError::Dead
}
