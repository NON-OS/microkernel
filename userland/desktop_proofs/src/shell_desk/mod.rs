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

//! The desktop shell's tray, scale, Launchpad and dialog rules, at the
//! `crate::` paths its files use.

// The desktop shell's tray table, under the `crate::protocol` path its files
// name the label size by. The real code's own style is allowed on the
// include rather than restyled here.
#[path = "../../../capsule_desktop_shell/src/protocol/limits.rs"]
pub mod protocol;

#[allow(clippy::new_without_default, clippy::result_unit_err)]
#[path = "../../../capsule_desktop_shell/src/state/tray/mod.rs"]
pub mod tray;

// The desktop shell's scale: the brand's rule by the canvas, and its type and
// geometry sizes taken through it.
#[path = "../../../capsule_desktop_shell/src/state/scale.rs"]
pub mod shell_scale;

#[path = "../../../capsule_desktop_shell/src/render/ui_font/mod.rs"]
pub mod ui_font;

/// How the wheel turns the Launchpad's pages.
#[path = "../../../capsule_desktop_shell/src/state/launchpad_wheel.rs"]
pub mod launchpad_wheel;

// When the desktop offers the installer: once, on a live session only.
#[path = "../../../capsule_desktop_shell/src/state/live_prompt.rs"]
pub mod live_prompt;

// What the shell holds of the router's grabs while a menu, a dialog, the
// Launchpad, a drag or a rename is up.
#[path = "../../../capsule_desktop_shell/src/state/grab_rule.rs"]
pub mod grab_rule;

// The keys a desktop dialog answers, and which button has the keyboard.
#[path = "../../../capsule_desktop_shell/src/state/dialog_keys.rs"]
pub mod dialog_keys;
