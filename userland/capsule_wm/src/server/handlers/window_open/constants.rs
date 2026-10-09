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

pub(super) const PLACEMENT_GAP: u32 = 24;
pub(super) const PLACEMENT_STEP: u32 = 40;

/// Height of the desktop menubar, which a window must not open underneath.
/// Mirrors the shell's `MENUBAR_H_LOGICAL` and the app skeleton's `chrome`
/// constant; a window placed above this is one whose titlebar cannot be
/// grabbed. Three crates, one number, and it must stay one number.
/// The desktop menubar at scale 1, the shell's `MENUBAR_H_LOGICAL`.
pub(super) const MENUBAR_H: u32 = 46;

#[path = "../../../../../capsule_install/brand/src/scale_rule.rs"]
mod scale_rule;

/// The bar as the shell draws it on a display of `width` by `height`: scaled
/// by the brand rule the shell uses and rounded as its `px` rounds. At scale 1
/// everywhere, a window placed on a 1920 by 1080 canvas opened twelve pixels
/// under the 58 pixel bar.
pub(crate) fn menubar_for(width: u32, height: u32) -> u32 {
    (MENUBAR_H * scale_rule::quarters_for(width, height) + 2) / 4
}

/// The dock's band at scale 1: the shell's 64 pixel dock and the 16 under it.
pub(super) const DOCK_BAND: u32 = 80;

/// The dock's band as the shell draws it. The dock is painted under every
/// window, so a new window is placed clear of it where it fits; one placed
/// over it hid the apps on it, the installer among them.
pub(crate) fn dock_for(width: u32, height: u32) -> u32 {
    (DOCK_BAND * scale_rule::quarters_for(width, height) + 2) / 4
}
