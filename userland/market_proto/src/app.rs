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

//! One listing in full, as `OP_GET_APP` returns it: its id, the capsule
//! measurement, the name, the publisher, the publisher's 32-byte key, the
//! description, and how many releases it has.

use alloc::vec::Vec;

use super::lp::{lp, skip, u32_at};

pub struct App {
    pub name: Vec<u8>,
    pub publisher: Vec<u8>,
    pub description: Vec<u8>,
    pub releases: u32,
}

pub fn parse_app(body: &[u8]) -> Option<App> {
    let (_, at) = lp(body, 0)?;
    let at = skip(body, at, 32)?;
    let (name, at) = lp(body, at)?;
    let (publisher, at) = lp(body, at)?;
    let at = skip(body, at, 32)?;
    let (description, at) = lp(body, at)?;
    // The release count closes the message.
    let releases = u32_at(body, at)?;
    Some(App {
        name: name.to_vec(),
        publisher: publisher.to_vec(),
        description: description.to_vec(),
        releases,
    })
}
