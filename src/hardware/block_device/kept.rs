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

//! The disk kept for this boot, and where its store and plan lie: chosen
//! once, then answered without asking any disk again.

use core::sync::atomic::{AtomicBool, Ordering};

use spin::Once;

use super::announce::announce;
use super::backend::Backend;
use super::gpt::Places;
use super::BlockDeviceError;

static SELECTED: Once<(Backend, Places)> = Once::new();
static TOLD_NONE: AtomicBool = AtomicBool::new(false);

/// The disk already kept for this boot, without asking any disk.
pub fn chosen() -> Option<Backend> {
    SELECTED.get().map(|k| k.0)
}

/// Where the kept disk's store and plan lie.
pub fn places() -> Option<Places> {
    SELECTED.get().map(|k| k.1)
}

pub(super) fn keep(b: Backend, p: Places) -> Backend {
    SELECTED.call_once(|| (announce(b, &p), p)).0
}

pub(super) fn none() -> BlockDeviceError {
    if !TOLD_NONE.swap(true, Ordering::Relaxed) {
        let line = "[BLOCK] no disk carries the NONOS store or disk plan yet; block I/O refused";
        crate::sys::serial::println(line.as_bytes());
        crate::log::warn!("{}", line);
    }
    BlockDeviceError::Dead
}

/// The end of a pass that found no boot disk: another NONOS disk is kept
/// once every disk answered, or once the wait for a refused one runs out.
pub(super) fn settle(
    other: Option<(Backend, Places)>,
    refused: Option<BlockDeviceError>,
) -> Result<Backend, BlockDeviceError> {
    match (other, refused) {
        (Some((b, p)), None) => Ok(keep(b, p)),
        (Some((b, p)), Some(_)) if super::wait::waited_for_boot_disk() => Ok(keep(b, p)),
        (Some(_), Some(e)) => Err(e),
        (None, _) => Err(none()),
    }
}
