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

//! Answering a thread the family held parked: with the value its call
//! returns, or by entering the handler of a signal it now takes.

use nonos_libc::mk_foreign_reply;

use crate::linux::guest::Guest;

/// Reply to a parked thread, or enter the handler of a signal it now takes.
pub fn answer(g: &mut Guest, tid: u32, value: u64) {
    if !super::deliver::maybe_deliver(g, tid, value) {
        let _ = mk_foreign_reply(tid, value);
    }
}
