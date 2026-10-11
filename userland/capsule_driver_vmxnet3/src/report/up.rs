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

//! The line a finished bring-up leaves: the link as GET_LINK answers.

use super::line::Line;
use crate::constants::cmd::LINK_UP;

pub fn up(link: u32) {
    let mut line = Line::new(b"driver.vmxnet3: up, link ");
    if link & LINK_UP == 0 {
        line.push(b"down");
    } else {
        line.push(b"up ");
        line.dec(link >> 16);
        line.push(b" Mb/s");
    }
    line.say();
}
