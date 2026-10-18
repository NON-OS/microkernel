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

//! A request for the service: an op and the body it carries, at most
//! FORGET's seven octets.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    pub op: u16,
    body: [u8; 7],
    len: usize,
}

impl Request {
    pub fn new(op: u16, body: &[u8]) -> Self {
        let len = body.len().min(7);
        let mut b = [0u8; 7];
        b[..len].copy_from_slice(&body[..len]);
        Self { op, body: b, len }
    }

    pub fn body(&self) -> &[u8] {
        &self.body[..self.len]
    }
}

/// What a key comes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Ask the service this.
    Ask(Request),
    /// The page changed by itself: the cursor, the digits or the notice.
    Shown,
    Close,
    NextSection,
    PrevSection,
    Nothing,
}
