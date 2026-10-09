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

//! Looking at each waiting connect once, and answering the ones that resolved.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::mk_time_millis;

use crate::clients::tcp;
use crate::state;

use super::settle;
use super::table::PENDING;
use super::verdict::{decide, Verdict};

/// Answer every connect whose handshake resolved or whose time ran out.
pub fn advance(tx: &mut [u8]) {
    let list = core::mem::take(&mut *PENDING.lock());
    if list.is_empty() {
        return;
    }
    let now = mk_time_millis();
    let mut still = Vec::new();
    for p in list {
        match decide(tcp::state(state::tcp(), p.transport), now, p.deadline_ms) {
            Verdict::Waiting => still.push(p),
            Verdict::Up => settle::established(p, tx),
            Verdict::Failed => settle::failed(p, tx),
        }
    }
    PENDING.lock().extend(still);
}
