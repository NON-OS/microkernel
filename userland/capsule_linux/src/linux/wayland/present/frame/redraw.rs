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

//! The frame painted again when only it changed: a control lit, a title.

use nonos_app_skeleton::clients::compositor::damage_commit;
use nonos_app_skeleton::discover::lookup_port;
use nonos_toolkit::decorations::DecorationHit;

use super::super::super::scene::Scene;
use super::super::super::window_damage::window_damage;
use super::now::shown;
use super::paint::{paint, Look};

/// The longest title kept, in bytes.
const TITLE_MAX: usize = 160;

pub fn redraw(scene: &mut Scene) {
    let Some((a, shape)) = shown(scene) else { return };
    if !a.framed() {
        return;
    }
    scene.frame.chrome = Some((a, scene.frame.hover, scene.keyboard_on.is_some()));
    let look = Look {
        title: &scene.frame.title,
        hover: scene.frame.hover,
        focused: scene.keyboard_on.is_some(),
    };
    let bytes = (a.w as usize) * (a.h as usize) * 4;
    let Some(out) = scene.pixels.frame(bytes) else { return };
    if !paint(out, a, &scene.frame.client, shape, (&look, true), scene.frame.quarters) {
        return;
    }
    let Some((x, y, w, h)) = window_damage(scene.at, a.w, a.h) else { return };
    if let Some(port) = lookup_port(b"compositor") {
        let rid = scene.next_serial();
        let _ = damage_commit(port, rid, x, y, w, h);
    }
}

/// The control under the pointer is now `hit`: the frame again if it changed.
pub fn light(scene: &mut Scene, hit: DecorationHit) {
    if scene.frame.hover != hit {
        scene.frame.hover = hit;
        redraw(scene);
    }
}

/// The guest named its window: the title bar says so.
pub fn title(scene: &mut Scene, title: &[u8]) {
    let title = title.strip_suffix(b"\0").unwrap_or(title);
    // Whole UTF-8 only, and no longer than a title bar could show.
    let mut end = title.len().min(TITLE_MAX);
    while end > 0 && core::str::from_utf8(&title[..end]).is_err() {
        end -= 1;
    }
    if scene.frame.title != title[..end] {
        scene.frame.title.clear();
        scene.frame.title.extend_from_slice(&title[..end]);
        redraw(scene);
    }
}
