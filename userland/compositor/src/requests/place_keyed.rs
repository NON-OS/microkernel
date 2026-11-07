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

//! The keyed scene requests: a layer named by its surface handle, one per
//! window, and the window manager's raise of one window.

use super::place::Placed;
use super::read::{wide, words};
use crate::protocol::{
    FOCUS_SET_WINDOW_REQ_LEN, SCENE_REMOVE_LAYER_REQ_LEN, SCENE_SUBMIT_LAYER_REQ_LEN,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlacedKeyed {
    pub placed: Placed,
    /// The window manager's id for the window the layer draws; never 0.
    pub window_id: u32,
    /// The handle of the layer this one takes the place of, if any.
    pub replaces: Option<u64>,
}

/// handle, x, y, width, height, z, window id, replaced handle (0 for none),
/// flags and a reserved word, both 0 in this version.
pub fn scene_submit_layer(body: &[u8]) -> Option<PlacedKeyed> {
    let [lo, hi, x, y, width, height, z, window_id, r_lo, r_hi, flags, reserved] =
        words::<12, SCENE_SUBMIT_LAYER_REQ_LEN>(body)?;
    let handle = wide(lo, hi);
    if handle == 0 || window_id == 0 || flags != 0 || reserved != 0 {
        return None;
    }
    let replaces = Some(wide(r_lo, r_hi)).filter(|&h| h != 0);
    let placed = Placed { handle, x, y, width, height, z };
    Some(PlacedKeyed { placed, window_id, replaces })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoveKeyed {
    pub handle: u64,
    pub z: u32,
}

/// handle, z, a reserved word 0.
pub fn scene_remove_layer(body: &[u8]) -> Option<RemoveKeyed> {
    let [lo, hi, z, reserved] = words::<4, SCENE_REMOVE_LAYER_REQ_LEN>(body)?;
    let handle = wide(lo, hi);
    (handle != 0 && reserved == 0).then_some(RemoveKeyed { handle, z })
}

/// owner pid and window id. A window id of 0 names the project's layers
/// submitted the old way, as FOCUS_SET does for that owner.
pub fn focus_set_window(body: &[u8]) -> Option<(u32, u32)> {
    let [pid, window_id] = words::<2, FOCUS_SET_WINDOW_REQ_LEN>(body)?;
    Some((pid, window_id))
}
