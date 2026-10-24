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

//! The window a manifest opens at on a `width` x `height` canvas, alone, so
//! a host proof sizes a window exactly as the runner does (fit_display.rs
//! asks the compositor for the canvas, then this).

use nonos_toolkit::decorations::chrome_growth_at;

use crate::app::{AppManifest, WindowKind};

use super::chrome::{dock_for, menubar_for, quarters_for};

// Largest a window opens relative to the display, as a percent. Below 100 so
// windows cascade with reachable titlebars instead of covering the screen.
const WINDOW_FRACTION: u32 = 88;

pub fn fit_manifest(mut manifest: AppManifest, width: u32, height: u32) -> AppManifest {
    if manifest.kind != WindowKind::Normal || width == 0 || height == 0 {
        return manifest;
    }
    let bar = menubar_for(width, height);
    let dock = dock_for(width, height);
    // The frame is drawn at the display's scale, so it takes more of the
    // window than it did at one to one. Grow the window by that much, so the
    // content area the app laid itself out for stays the size it asked for.
    let (grow_w, grow_h) = chrome_growth_at(quarters_for(width, height));
    manifest.width = manifest.width.saturating_add(grow_w);
    manifest.height = manifest.height.saturating_add(grow_h);
    // Cap a window to a fraction of the work area, between the menubar and
    // the dock, so two windows never fully occlude each other and the desktop
    // plus dock stay visible. The window manager cascades them from here, and
    // a visible titlebar is what makes click-to-raise a real way to switch.
    let work_h = height.saturating_sub(bar + dock);
    let max_w = (width * WINDOW_FRACTION / 100).max(480);
    let max_h = (work_h * WINDOW_FRACTION / 100).max(360);

    let shrunk = manifest.width > max_w || manifest.height > max_h;
    manifest.width = manifest.width.min(max_w);
    manifest.height = manifest.height.min(max_h);

    if shrunk {
        // The manifest's origin was chosen for the larger size; centre instead.
        manifest.initial_x = width.saturating_sub(manifest.width) / 2;
        manifest.initial_y = bar + work_h.saturating_sub(manifest.height) / 2;
    } else {
        // Keep the requested origin but pull the window fully on screen.
        manifest.initial_x = manifest.initial_x.min(width.saturating_sub(manifest.width));
        let lowest = height.saturating_sub(dock + manifest.height).max(bar);
        manifest.initial_y = manifest.initial_y.max(bar).min(lowest);
    }
    manifest
}
