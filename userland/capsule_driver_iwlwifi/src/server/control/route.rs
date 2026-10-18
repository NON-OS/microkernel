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

//! Where a request goes: the link, the radio's join, or the control family.

use super::kinds::{NNET_MAGIC, OP_CONNECT, OP_DISCONNECT, OP_LINK, WIFI_HDR, WIFI_MAGIC};

/// Where a request goes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    /// net_core's link protocol: the radio's link.
    Link,
    /// A join op (connect, disconnect, link) the radio runs.
    Join(u16),
    /// The rest of the control family, answered by `answer`.
    Control,
    /// Neither: answered with nothing.
    Other,
}

/// Where `req` goes; `can_join` is whether the radio runs joins now.
pub fn route(req: &[u8], can_join: bool) -> Route {
    let Some(magic) = req.get(..4) else { return Route::Other };
    match u32::from_le_bytes([magic[0], magic[1], magic[2], magic[3]]) {
        NNET_MAGIC => Route::Link,
        WIFI_MAGIC if req.len() >= WIFI_HDR => {
            let op = u16::from_le_bytes([req[4], req[5]]);
            if can_join && matches!(op, OP_CONNECT | OP_DISCONNECT | OP_LINK) {
                Route::Join(op)
            } else {
                Route::Control
            }
        }
        _ => Route::Other,
    }
}
