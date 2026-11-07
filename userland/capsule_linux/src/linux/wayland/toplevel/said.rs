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

//! What a toplevel says of itself that shapes the size it is offered first
//! (size.rs): whether it has a parent, and its least and most size. Kept on
//! its surface; later changes are the client's own to keep to.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::shell::{TOPLEVEL_SET_MAX_SIZE, TOPLEVEL_SET_MIN_SIZE, TOPLEVEL_SET_PARENT};

/// set_parent, set_min_size or set_max_size on `toplevel`.
pub fn said(guest: &mut Guest, toplevel: u32, op: u16, args: &mut Args<'_>) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.toplevel == Some(toplevel)) else {
        return;
    };
    let (a, b) = (args.u32().unwrap_or(0), args.u32().unwrap_or(0));
    match op {
        TOPLEVEL_SET_PARENT => s.parent = a != 0,
        TOPLEVEL_SET_MIN_SIZE => s.limits.0 = (a, b),
        TOPLEVEL_SET_MAX_SIZE => s.limits.1 = (a, b),
        _ => {}
    }
}
