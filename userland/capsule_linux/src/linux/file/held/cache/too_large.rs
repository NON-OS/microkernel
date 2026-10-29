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

//! The person told, once a run, when an app writes a file past the most one
//! file may hold here (table.rs MAX_FILE). Without it a wallet says only that
//! its save failed, and nothing says why: EFBIG reaches the app alone.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::linux::abi::errno;

static TOLD: AtomicBool = AtomicBool::new(false);

const LINE: &[u8] = b"linux: this app wrote a file past 64 MiB, the most one file may hold \
    here; that write was refused.\n";

/// EFBIG, said once a run.
pub(super) fn too_large() -> i64 {
    if !TOLD.swap(true, Ordering::Relaxed) {
        crate::linux::say::say(LINE);
    }
    errno::EFBIG
}
