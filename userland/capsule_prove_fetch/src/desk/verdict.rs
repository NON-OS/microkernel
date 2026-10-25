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

//! What the relay's answer means. An upload is held at 200 or 201; a
//! download at 200, its bytes still to be checked against the anchor; a 404
//! is ENOENT. A 5xx is worth another connection; anything else is RELAY.
//! `None` is "try another connection", `Some` the op's end.

use alloc::vec::Vec;

use nonos_proof_abi::wire::fetch::RELAY;

use super::errno::ENOENT;

pub fn verdict(up: bool, code: u16, body: Vec<u8>) -> Result<Vec<u8>, Option<i32>> {
    match (up, code) {
        (true, 200 | 201) => Ok(Vec::new()),
        (false, 200) => Ok(body),
        (false, 404) => Err(Some(ENOENT)),
        (_, 500..=599) => Err(None),
        _ => Err(Some(RELAY)),
    }
}
