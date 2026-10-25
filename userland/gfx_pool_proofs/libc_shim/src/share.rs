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

//! A share: the handle, the receiver's right recorded, none for the
//! caller's own process (the kernel's `receiver`).

use std::cell::Cell;

use crate::registry::{LIVE, NO_SHARE};
use crate::service::OWN_PID;

pub fn mk_surface_share(sid: u64, pid: u32, flags: u64) -> i64 {
    LIVE.with(|l| {
        let mut l = l.borrow_mut();
        let Some(e) = l.iter_mut().find(|e| e.sid == sid) else { return -22 };
        match pid {
            0 => -22,
            OWN_PID => e.handle as i64,
            _ if NO_SHARE.with(Cell::get) => -12,
            _ => {
                // Named again, a receiver keeps its one right (Grant::Already).
                if !e.shares.iter().any(|&(p, _)| p == pid) {
                    e.shares.push((pid, flags));
                }
                e.handle as i64
            }
        }
    })
}
