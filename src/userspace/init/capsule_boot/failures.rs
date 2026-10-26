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

//! How many of the boot's capsules failed to spawn. A capsule this image was
//! built without, or one its profile refuses, is a choice and is not counted;
//! every other error is, whether verification, attestation, loading or
//! registration refused it. init reads the count once its spawns are done.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

static FAILED: AtomicU32 = AtomicU32::new(0);

pub(super) fn note(err: SpawnError) {
    if matches!(err, SpawnError::FeatureDisabled | SpawnError::ProfileRefused) {
        return;
    }
    let _ = FAILED.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1));
}

/// The boot's failed spawns so far.
pub(crate) fn failed() -> u32 {
    FAILED.load(Ordering::Relaxed)
}
