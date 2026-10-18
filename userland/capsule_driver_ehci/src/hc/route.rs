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

//! Deciding a connected port: a low-speed device (K-state on the line)
//! goes to the companion at once; any other is reset, and a port left
//! disabled after reset holds a full-speed device, which goes to the
//! companion too. Only a high-speed device stays (EHCI 1.0, as recalled).

use super::port::{hand, offset, reset};
use crate::env::{Clock, Regs};
use crate::facts::port::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    Empty,
    Companion,
    High,
}

pub fn route<R: Regs, C: Clock>(r: &mut R, c: &C, op: u32, port: u8) -> Route {
    let off = offset(op, port);
    let raw = r.r32(off);
    if raw & PORT_OWNER != 0 {
        return Route::Companion;
    }
    if raw & PORT_CCS == 0 {
        return Route::Empty;
    }
    if (raw & PORT_LS_MASK) >> PORT_LS_SHIFT == LS_K {
        hand(r, off);
        return Route::Companion;
    }
    match reset(r, c, off) {
        Ok(true) => Route::High,
        _ => {
            hand(r, off);
            Route::Companion
        }
    }
}
