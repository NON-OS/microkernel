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

//! Where a connection is being made to, and how far it has got.

use alloc::string::String;

use super::Phase;

pub struct Dial {
    pub host: String,
    pub port: u16,
    pub ip: Option<[u8; 4]>,
    /* The connect was started; now it is polled. */
    pub sent: bool,
    /* A tick has passed, so "Connecting to" has been drawn. */
    pub shown: bool,
    /* The phase the fetch takes once connected. */
    pub then: Phase,
}

impl Dial {
    pub fn new(host: &str, port: u16, then: Phase) -> Dial {
        Dial { host: String::from(host), port, ip: None, sent: false, shown: false, then }
    }
}
