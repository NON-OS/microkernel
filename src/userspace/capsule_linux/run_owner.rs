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

//! Who holds the one Linux run, and what it was started for.

use alloc::string::String;

use super::roles::RUN;
use super::run_net::RUN_NET;
use crate::services::registry::lookup_service;

/// Whether a store-started or windowed Linux program still holds a run
/// role: its endpoint is released when the process ends, so a second open
/// before then is told it is busy instead of being queued and refused unseen.
/// Either role counts, as there is one run at a time.
pub fn run_busy() -> bool {
    lookup_service(RUN.name).is_some() || run_net_busy()
}

/// Whether a run already holds the network role.
pub fn run_net_busy() -> bool {
    lookup_service(RUN_NET.name).is_some()
}

/// The package `pid` was started to run, when `pid` is the process the
/// kernel registered as app.linux.run at spawn, which no other capsule may
/// claim. Its argv is init's, `run` then the package (install.rs), and the
/// process cannot change it.
pub fn run_package_of(pid: u32) -> Option<String> {
    if lookup_service(RUN.name)?.pid != pid {
        return None;
    }
    crate::process::with_process(pid, |pcb| match pcb.argv.lock().as_slice() {
        [verb, package] if verb == "run" => Some(package.clone()),
        _ => None,
    })
    .flatten()
}
