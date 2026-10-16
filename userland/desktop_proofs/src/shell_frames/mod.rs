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

//! How the shell's frames reach their surfaces, its toasts, and the system
//! keys the router hands it.

// How the shell's chrome and desk frames reach their surfaces: drawn off
// screen and copied over after, so a composite never reads half a frame.
#[path = "../../../capsule_desktop_shell/src/render/whole_frame.rs"]
pub mod whole_frame;

// How far a shadowed panel's damage must reach.
#[path = "../../../capsule_desktop_shell/src/render/shadow_reach.rs"]
pub mod shadow_reach;

// The toast queue, whose generation tells the shell when its chrome frame
// no longer shows the toasts it holds. A toast's tone is marked through
// `crate::sound`, which plays nothing here.
#[path = "."]
pub mod toast_state {
    pub use crate::store_state::NotifyLevel;
    #[path = "../../../capsule_desktop_shell/src/state/toast/mod.rs"]
    pub mod toast;
    #[path = "../../../capsule_desktop_shell/src/state/toasts/mod.rs"]
    pub mod toasts;
}

pub mod sound {
    pub fn mark(_level: crate::toast_state::NotifyLevel) {}
}

#[path = "../../../capsule_desktop_shell/src/vfs_client/constants.rs"]
pub mod vfs_constants;

// What the shell does with the system keys the input router hands it whatever
// has focus: the volume keys' steps, the request they send and the notice they
// put up, and the power key. Beside them, the router's own list of those keys.
#[path = "."]
pub mod shell_system_keys {
    #[path = "../../../capsule_desktop_shell/src/state/brightness.rs"]
    pub mod brightness;
    #[path = "../../../capsule_desktop_shell/src/state/lid.rs"]
    pub mod lid;
    #[path = "../../../capsule_desktop_shell/src/state/system_key.rs"]
    pub mod system_key;
    #[path = "../../../capsule_desktop_shell/src/state/volume/mod.rs"]
    pub mod volume;
}

#[path = "../../../capsule_input_router/src/route/shell_keys.rs"]
pub mod router_shell_keys;
