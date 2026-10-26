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
//! What a window in the strip looks like: a dock app's own tile and name, a
//! capsule-store app's generated tile and its name, or a plain tile.

use crate::state::ctx::switch::Win;
use crate::state::{Context, LAUNCHER_APPS};

pub(super) enum Face<'a> {
    App(usize),
    Named(&'a [u8]),
}

/// The name of a window no app table names.
const UNNAMED: &[u8] = b"Window";

pub(super) fn face(ctx: &Context, w: Win) -> Face<'_> {
    let tracked = ctx.taskbar.windows.iter().find(|t| t.pid == w.0 && t.window_id == w.1);
    if let Some(t) = tracked.filter(|t| (t.app as usize) < LAUNCHER_APPS.len()) {
        return Face::App(t.app as usize);
    }
    let installed = ctx.store_apps.installed_pids.iter().find(|(_, &pid)| pid == w.0);
    Face::Named(installed.map_or(UNNAMED, |(name, _)| name.as_slice()))
}
