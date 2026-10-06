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

//! Desktop chrome an application has to work around.
//!
//! One definition. The menubar height was written out separately in the drag
//! code and the maximise code, both as 28, while the shell actually draws 46.
//! So a window dragged to the top slid eighteen pixels under the bar, and a
//! maximised one left a strip of desktop showing above it. Three numbers for
//! one bar is three chances to be wrong about it.

use core::sync::atomic::{AtomicU32, Ordering};

/// Height of the desktop menubar at scale 1, the shell's own
/// `MENUBAR_H_LOGICAL`. If the shell's bar changes, this changes with it.
pub const MENUBAR_H: u32 = 46;

// The brand's one rule, included by every program that sizes by it. A proof
// crate that wants this file beside the shell's scale takes it through the
// app_skeleton crate, so the rule is never loaded twice in one crate.
#[path = "../../../capsule_install/brand/src/scale_rule.rs"]
mod scale_rule;

/// The bar as the shell draws it on this display: scaled by the brand rule
/// the shell uses, rounded the way the shell's `px` rounds. Until the display
/// is known it is the bar at scale 1.
static BAR: AtomicU32 = AtomicU32::new(MENUBAR_H);

/// Height of the desktop menubar on this display.
///
/// It was the bar at scale 1 everywhere. Once the shell scaled its bar, 58
/// pixels on a 1920 by 1080 canvas, a window opened or dragged to the top slid
/// twelve pixels under it, and its close button with it.
pub fn menubar_h() -> u32 {
    BAR.load(Ordering::Relaxed)
}

/// The bar for a display of `width` by `height`.
pub fn menubar_for(width: u32, height: u32) -> u32 {
    (MENUBAR_H * scale_rule::quarters_for(width, height) + 2) / 4
}

/// The band the dock takes at the foot of the screen at scale 1: the shell's
/// dock, 64, and the gap under it, 16 (render/layout.rs).
pub const DOCK_BAND: u32 = 80;

/// The dock's band as the shell draws it on this display.
static DOCK: AtomicU32 = AtomicU32::new(DOCK_BAND);

/// The band at the foot of the screen a new window stays out of. The dock is
/// drawn under every window, so a window opened over it covered the dock and
/// the apps on it, the installer among them, until it was moved.
pub fn dock_band() -> u32 {
    DOCK.load(Ordering::Relaxed)
}

/// The dock's band for a display of `width` by `height`.
pub fn dock_for(width: u32, height: u32) -> u32 {
    (DOCK_BAND * scale_rule::quarters_for(width, height) + 2) / 4
}

/// The display's scale in quarters, the shell's own (4 is one to one).
static QUARTERS: AtomicU32 = AtomicU32::new(4);

/// The scale this window's frame is drawn and hit tested at. The shell draws
/// its bar, dock and type by it, so a frame left at one to one was a thin
/// strip with buttons too small to hit beside them on a large display.
pub fn quarters() -> u32 {
    QUARTERS.load(Ordering::Relaxed)
}

/// The scale for a display of `width` by `height`.
pub fn quarters_for(width: u32, height: u32) -> u32 {
    scale_rule::quarters_for(width, height)
}

/// Note the display, once its size is known.
pub fn learn_display(width: u32, height: u32) {
    BAR.store(menubar_for(width, height), Ordering::Relaxed);
    DOCK.store(dock_for(width, height), Ordering::Relaxed);
    QUARTERS.store(quarters_for(width, height), Ordering::Relaxed);
}

/// The work area on a `width` by `height` display: between the menubar and
/// the dock's band. New windows are sized to fit in it and the window
/// manager cascades them inside it, so a window opens clear of the dock.
///
/// It was also the rect a maximised window took, which left the dock's band
/// and the dock showing under a window meant to fill the screen.
pub fn work_area(width: u32, height: u32) -> (u32, u32, u32, u32) {
    let bar = menubar_for(width, height);
    let dock = dock_for(width, height);
    let h = height.saturating_sub(bar + dock).max(1);
    (0, bar, width, h)
}

/// The rect the green button gives a window on a `width` by `height`
/// display: full screen, the whole width from the foot of the menubar down
/// to the bottom edge, over the band the dock is drawn in. The desktop shell
/// hides the dock while such a window shows, and brings it over the window
/// when the pointer touches the bottom edge (docs/handbook/desktop/
/// 0.9.2-notes.md). The menubar stays, with its menus and clock.
///
/// It was the work area, which stopped above the dock's band, so a
/// "maximised" window left a band of desktop and the dock under it.
pub fn full_screen(width: u32, height: u32) -> (u32, u32, u32, u32) {
    let bar = menubar_for(width, height).min(height.saturating_sub(1));
    (0, bar, width, height - bar)
}
