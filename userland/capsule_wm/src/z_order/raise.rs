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

use super::ZStack;
use crate::window::table::PER_OWNER;
use crate::window::{Window, WindowTable};

/// Put a window on top of the stack, with the popups open over it on top of
/// it in their own order. Some(true) when that changed the order, that is
/// when a window not its own popup sat above it; Some(false) when it was
/// already on top and keeps its z; None when the table has no such window.
/// A popup is raised by its parent's raise: asked to raise one, this raises
/// the window it belongs to.
///
/// Every raise goes through here so the caller learns whether the order moved,
/// and tells the compositor exactly then: the compositor stacks layers by the
/// raises it is told about, and a raise it never heard of is a window drawn
/// under the one the hit test gives the next click to.
pub fn raise(
    windows: &mut WindowTable,
    z: &mut ZStack,
    owner_pid: u32,
    window_id: u32,
) -> Option<bool> {
    let root = root_of(windows, owner_pid, window_id)?;
    let current = windows.find(owner_pid, root)?.z;
    let mut popups = [0u32; PER_OWNER];
    let n = windows.subtree(owner_pid, root, &mut popups);
    let own = |w: &Window| {
        w.owner_pid == owner_pid && (w.window_id == root || popups[..n].contains(&w.window_id))
    };
    let covered = windows.windows().any(|w| !own(w) && w.z > current);
    if !covered {
        return Some(false);
    }
    let top = z.allocate();
    windows.find_mut(owner_pid, root)?.z = top;
    let mut order = [(0u32, 0u32); PER_OWNER];
    for (slot, &id) in order.iter_mut().zip(popups[..n].iter()) {
        *slot = (windows.find(owner_pid, id).map_or(0, |w| w.z), id);
    }
    order[..n].sort_unstable();
    for &(_, id) in order[..n].iter() {
        let next = z.allocate();
        if let Some(w) = windows.find_mut(owner_pid, id) {
            w.z = next;
        }
    }
    Some(true)
}

/// The window a popup belongs to, through any popups between; a window that
/// is not a popup is its own.
pub fn root_of(windows: &WindowTable, owner_pid: u32, window_id: u32) -> Option<u32> {
    let mut at = windows.find(owner_pid, window_id)?;
    for _ in 0..PER_OWNER {
        if !at.is_popup() {
            return Some(at.window_id);
        }
        at = windows.find(owner_pid, at.parent)?;
    }
    None
}

/// The window and its popups in stack order, bottom first, into `out`; how
/// many. The compositor is told of each in this order after a raise, so the
/// layers it draws stack as this table does.
pub fn chain(windows: &WindowTable, owner_pid: u32, window_id: u32, out: &mut [u32]) -> usize {
    let Some(root) = root_of(windows, owner_pid, window_id) else { return 0 };
    let mut popups = [0u32; PER_OWNER];
    let n = windows.subtree(owner_pid, root, &mut popups);
    let mut order = [(0u32, 0u32); PER_OWNER + 1];
    order[0] = (windows.find(owner_pid, root).map_or(0, |w| w.z), root);
    for (slot, &id) in order[1..].iter_mut().zip(popups[..n].iter()) {
        *slot = (windows.find(owner_pid, id).map_or(0, |w| w.z), id);
    }
    order[..n + 1].sort_unstable();
    let mut k = 0;
    for &(_, id) in order[..n + 1].iter() {
        if k < out.len() {
            out[k] = id;
            k += 1;
        }
    }
    k
}
