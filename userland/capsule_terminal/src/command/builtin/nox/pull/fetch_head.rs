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

//! One HEAD exchange: the length a 200 says its body has.

use alloc::vec::Vec;

use super::super::conn::{Rx, Transport};
use super::super::http;
use super::{CLOSE_WAIT, EMPTY_BUDGET, MAX_FETCH};

pub(super) fn head_len<T: Transport>(
    conn: &T,
    host: &[u8],
    path: &[u8],
    extra: &[u8],
) -> Option<usize> {
    if !conn.send(&http::build_head(host, path, extra)) {
        return None;
    }
    let mut raw = Vec::new();
    let mut empties = 0u32;
    loop {
        if let Some(head) = http::parse_head(&raw) {
            if head.status != 200 {
                return None;
            }
            return head.content_length;
        }
        match conn.recv(&mut raw) {
            Rx::Data => empties = 0,
            Rx::Empty => {
                if conn.state() >= CLOSE_WAIT {
                    return None;
                }
                empties += 1;
                if empties > EMPTY_BUDGET {
                    return None;
                }
            }
            Rx::Gone => return None,
        }
        if raw.len() > MAX_FETCH {
            return None;
        }
    }
}
