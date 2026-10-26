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

//! A run on its way to the device, sent with no lock held: the send is a
//! round trip to the block capsule, and its caller can sleep or be killed
//! there. A run is moved out from under PENDING first, so whatever happens
//! to the sender, PENDING is free and holds none of its sectors.

use alloc::vec::Vec;

use super::map_block::map_block_error;
use super::pending::{Run, PENDING};
use super::CryptoBlockError;

/// The whole run, leaving an empty one in its place.
pub(super) fn take(run: &mut Run) -> Run {
    Run { start: run.start, bytes: core::mem::take(&mut run.bytes) }
}

/// Send a run already moved out of PENDING. A run that failed is not
/// retried from here: its caller hears of it and sends nothing else.
pub(super) fn send(run: Run) -> Result<(), CryptoBlockError> {
    if run.bytes.is_empty() {
        return Ok(());
    }
    super::device::write(run.start, &run.bytes).map_err(map_block_error)
}

/// Drop whatever is held, unsent; whether anything was.
pub(super) fn drop_held() -> bool {
    let _quiet = crate::services::lifecycle::quiet();
    let mut run = crate::smp::lock_responsive(&PENDING);
    let had = !run.bytes.is_empty();
    run.bytes = Vec::new();
    had
}
