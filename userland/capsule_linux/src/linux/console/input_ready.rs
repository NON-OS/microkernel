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

//! Whether a read of the console's input would wait, and how much it would
//! find: poll, select, epoll and FIONREAD, each looking at the inbox first.

use super::input::pull;
use super::state::{attached, with};

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;

/// The console input's poll bits: readable once a read would not wait.
pub fn bits() -> u16 {
    if !attached() {
        return POLLIN | POLLOUT;
    }
    pull();
    match with(|c| c.queue.ready()) {
        true => POLLIN | POLLOUT,
        false => POLLOUT,
    }
}

/// Bytes a read would find now, as FIONREAD counts them.
pub fn queued() -> u64 {
    pull();
    with(|c| c.queue.queued() as u64)
}
