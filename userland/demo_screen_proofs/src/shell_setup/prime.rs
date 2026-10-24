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

//! The shell's setup/prime/mod.rs, declared here with the overlay, the peers
//! and the run module open to the crate.

#[path = "../../../capsule_desktop_shell/src/setup/prime/bands.rs"]
pub(crate) mod bands;
#[path = "../../../capsule_desktop_shell/src/setup/prime/open_chrome_windows.rs"]
mod open_chrome_windows;
#[path = "../../../capsule_desktop_shell/src/setup/prime/overlay.rs"]
pub(crate) mod overlay;
#[path = "../../../capsule_desktop_shell/src/setup/prime/patient.rs"]
pub(crate) mod patient;
#[path = "../../../capsule_desktop_shell/src/setup/prime/peers.rs"]
pub(crate) mod peers;
#[path = "../../../capsule_desktop_shell/src/setup/prime/register.rs"]
pub(crate) mod register;
#[path = "run.rs"]
pub(crate) mod run;
#[path = "../../../capsule_desktop_shell/src/setup/prime/wallpaper_policy.rs"]
pub mod wallpaper_policy;

pub use run::{run, subscribe_input, subscribe_wm};
