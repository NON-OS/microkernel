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

//! What the shell does with desktop files, pastes, launches and tool tiles,
//! what it says when they fail, its clock line and its quiet-service tick.

// Where desktop file actions land, and the question a Delete asks first.
#[path = "../../../capsule_desktop_shell/src/server/desktop/home.rs"]
pub mod desktop_home;

#[path = "../../../capsule_desktop_shell/src/state/delete_prompt.rs"]
pub mod delete_prompt;

// Clipboard text into the shell's one-line fields.
#[path = "../../../capsule_desktop_shell/src/state/paste_line.rs"]
pub mod paste_line;

// A Launchpad launch the installer is still loading, followed on the shell's
// turns instead of waited out on its frame loop. Its source names `alloc::`
// as no_std code does.
#[path = "../../../capsule_desktop_shell/src/state/launch/mod.rs"]
pub mod launch;

// What the shell's toasts say when something did not happen (a launch, a
// package, a desktop file action), and its battery label, which is nothing
// without a reading.
#[path = "../../../capsule_desktop_shell/src/state/says/mod.rs"]
pub mod says;

#[path = "../../../capsule_desktop_shell/src/state/indicators/battery_text/mod.rs"]
pub mod battery_text;

// What a Launchpad tool tile hands the Terminal (run, or typed for its
// arguments), kept apart from the paths Open With leaves; and the generated
// table of tool tiles it is decided for.
#[path = "../../../capsule_desktop_shell/src/state/open_arg/mod.rs"]
pub mod open_arg;

#[path = "../../../capsule_desktop_shell/src/state/tool_apps.rs"]
pub mod tool_apps;

// The menubar's date and time line, 24 and 12 hour.
#[path = "../../../capsule_desktop_shell/src/state/indicators/clock/mod.rs"]
pub mod clock_line;

/// What the desktop says about the store vfs loaded, beside the toast levels
/// its file names by `super::NotifyLevel`.
#[path = "."]
pub mod store_state {
    #[path = "../../../capsule_desktop_shell/src/state/notify.rs"]
    pub mod notify;
    pub use notify::NotifyLevel;
    #[path = "../../../capsule_desktop_shell/src/state/store_word.rs"]
    pub mod store_word;
}

// How long the shell's tick lets a quiet service be (policy, installer), and
// when the tick itself is due on the uptime clock.
#[path = "../../../capsule_desktop_shell/src/state/quiet_gap.rs"]
pub mod quiet_gap;
#[path = "../../../capsule_desktop_shell/src/server/runner/tick.rs"]
pub mod shell_tick;
