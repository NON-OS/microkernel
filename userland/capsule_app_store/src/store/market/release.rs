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

//! The release a listing would install: which version, and what the
//! operator recorded when it checked the bytes.

use nonos_market_proto::{pair_body, parse_release, OP_GET_RELEASE};

pub use nonos_market_proto::Release;

use super::failure::Failure;
use super::wire::exchange;

/// The default release: an empty id asks for it.
pub fn fetch(port: u32, request_id: u32, listing: &[u8]) -> Result<Release, Failure> {
    let body = exchange(port, OP_GET_RELEASE, request_id, &pair_body(listing, b""))?;
    parse_release(&body).ok_or(Failure::Malformed)
}
