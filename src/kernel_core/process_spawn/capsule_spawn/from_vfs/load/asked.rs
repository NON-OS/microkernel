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

//! A load asks only for capabilities its capsule's manifest offers. The
//! installer asks for the manifest's own set when its sender names none, so
//! a request beyond the manifest is a sender choosing capabilities: it is
//! refused, never narrowed, with one serial line.

use super::super::error::LoadError;
use crate::security::capsule_manifest::CapsuleManifest;

/// The bits `requested` names that the manifest neither requires nor offers.
pub(crate) fn beyond(manifest: &CapsuleManifest, requested: u64) -> u64 {
    requested & !(manifest.required_caps | manifest.optional_caps)
}

/// The requested set, refused with one line when it reaches past the manifest.
pub(crate) fn within_manifest(
    manifest: &CapsuleManifest,
    requested: u64,
    name: &str,
) -> Result<u64, LoadError> {
    let extra = beyond(manifest, requested);
    if extra == 0 {
        return Ok(requested);
    }
    let caller = crate::process::current_pid().unwrap_or(0);
    let line = alloc::format!(
        "[CAP-DENY] load: caller pid={caller} child={name} asked beyond its manifest {extra:#x}"
    );
    crate::sys::serial::println(line.as_bytes());
    Err(LoadError::BeyondParent)
}
