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

//! Fit a window's initial geometry to the real display. Manifests are written
//! against an assumed screen; on a smaller panel an oversized window would
//! cover everything and an off-screen origin would hide it. Clamp the size to
//! the work area (with a small margin so the desktop stays visible) and pull
//! the origin back so the whole window is on screen, centring when it had to
//! shrink. Applies to normal windows only; docks and overlays place themselves.

use crate::app::{AppManifest, WindowKind};
use crate::clients::compositor;
use crate::discover::Peers;

use super::fit_rule::fit_manifest;
use super::request_id::next;

pub(super) fn fit_to_display(
    manifest: AppManifest,
    peers: &Peers,
    request_id: &mut u32,
) -> AppManifest {
    if manifest.kind != WindowKind::Normal {
        return manifest;
    }
    let Ok(di) = compositor::display_info(peers.compositor, next(request_id)) else {
        return manifest;
    };
    if di.width == 0 || di.height == 0 {
        return manifest;
    }
    super::chrome::learn_display(di.width, di.height);
    fit_manifest(manifest, di.width, di.height)
}
