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

//! The capsule store's apps as the shell knows them: the installer's list,
//! the packages in /pkgs, the windows it launched, and the launch loading.

pub struct StoreApps {
    /// When the tick may next ask the installer for its apps, likewise.
    pub installer_gap: crate::state::quiet_gap::QuietGap,
    /// Base names of the capsule-store apps the installer reports, already
    /// filtered of anything impersonating a built-in. Loaded lazily once the
    /// installer service is up, then left alone.
    pub installed_apps: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    pub installed_apps_loaded: bool,
    /// File names of the `.nonos` packages sitting in /pkgs, rescanned every
    /// tick so one dropped in after boot appears without a restart.
    pub pkg_files: alloc::vec::Vec<alloc::string::String>,
    pub pkg_files_loaded: bool,
    /// Pid the installer handed back for each store app we have launched, so a
    /// second click focuses that window instead of loading another copy. An
    /// entry is dropped once its pid stops accepting control frames.
    pub installed_pids: alloc::collections::BTreeMap<alloc::vec::Vec<u8>, u32>,
    /// The Launchpad launch the installer is still loading, followed on the
    /// shell's turns (`server/handlers/installed_launch_poll.rs`).
    pub launch: Option<crate::state::launch::Launch>,
}

impl StoreApps {
    pub fn new() -> Self {
        StoreApps {
            installer_gap: crate::state::quiet_gap::QuietGap::new(),
            installed_apps: alloc::vec::Vec::new(),
            installed_apps_loaded: false,
            pkg_files: alloc::vec::Vec::new(),
            pkg_files_loaded: false,
            installed_pids: alloc::collections::BTreeMap::new(),
            launch: None,
        }
    }
}
