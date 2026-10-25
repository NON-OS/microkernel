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

//! A loaded capsule never holds more than the process it is started for.
//! The child's installed capabilities must lie within its parent's and the
//! ambient set (`AMBIENT_CAPS`, which every process has), unless the parent is
//! a launcher: it holds SpawnWindow, or Admin standing in as in
//! `can_spawn_window`. The parent is the one the load is attributed to: the
//! requester, when the installer loads on its behalf, else the caller. A
//! broker (SpawnBroker) loads only for someone else: a load from it naming no
//! attested parent, or itself, is refused, so it never lends its own
//! capabilities. A start outside the bound is refused, never narrowed.
//! Known edge: the child's set is weighed before the boot profile narrows it,
//! the parent's after, so on an air-gapped boot a parent that is no launcher
//! is refused a child needing Network. It fails closed.

use super::super::error::LoadError;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::AttestedParent;
use crate::process::core::AMBIENT_CAPS;
use crate::security::capsule_manifest::CapsuleManifest;

/// The bits `child` would install that `parent` lacks; zero when allowed.
pub(crate) fn missing(child: u64, parent: u64) -> u64 {
    let launcher = Capability::SpawnWindow.bit() | Capability::Admin.bit();
    if parent & launcher != 0 {
        return 0;
    }
    child & !(parent | AMBIENT_CAPS)
}

/// What the manifest installs for `requested`, as the verified spawn will.
pub(crate) fn installed(manifest: &CapsuleManifest, requested: u64) -> u64 {
    manifest.required_caps | (manifest.optional_caps & requested)
}

/// Refuses the load, with one serial line, when it would exceed its parent.
pub(crate) fn check(
    manifest: &CapsuleManifest,
    requested: u64,
    on_behalf_of: Option<AttestedParent>,
    name: &str,
) -> Result<(), LoadError> {
    let caller = crate::process::current_pid().ok_or(LoadError::BeyondParent)?;
    let broker = crate::process::caps::has(caller, Capability::SpawnBroker.bit());
    if broker && on_behalf_of.map_or(true, |p| p.pid() == caller) {
        crate::sys::serial::println(b"[CAP-DENY] load: a broker named no attested parent");
        return Err(LoadError::BeyondParent);
    }
    let parent = on_behalf_of.map(AttestedParent::pid).unwrap_or(caller);
    let held = crate::process::caps::bits(parent).ok_or(LoadError::BeyondParent)?;
    let gap = missing(installed(manifest, requested), held);
    if gap == 0 {
        return Ok(());
    }
    let line = alloc::format!(
        "[CAP-DENY] load: caller pid={caller} parent pid={parent} child={name} \
         missing caps {gap:#x}"
    );
    crate::sys::serial::println(line.as_bytes());
    Err(LoadError::BeyondParent)
}
