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

//! Host stand-in for push/request.rs, which sends one PUT over a connection.

// Replaces request::put: the PUT is recorded (its path and body) and said
// sent, so a test sees what would have gone on the wire.
use std::cell::RefCell;

thread_local! {
    static PUTS: RefCell<Vec<(Vec<u8>, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
}

pub fn put(
    _ip: [u8; 4],
    _port: u16,
    _host: &[u8],
    path: &[u8],
    body: &[u8],
    _extra: &[u8],
) -> Result<(), &'static str> {
    PUTS.with(|p| p.borrow_mut().push((path.to_vec(), body.to_vec())));
    Ok(())
}

/// Each PUT sent, as (path, body).
pub fn puts() -> Vec<(Vec<u8>, Vec<u8>)> {
    PUTS.with(|p| p.borrow().clone())
}
