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

//! The frame around every message the core sends on its WebSocket:
//! `{"corrId":"<id>","resp":{...}}` for an answer to a command, and the same
//! without `corrId` for an event the core pushes.

use alloc::string::String;

use super::json::{parse, Value};
use super::resp::Resp;
use super::resp_read::resp;

#[derive(Clone, Debug, PartialEq)]
pub struct Incoming {
    /// The `corrId` of the command this answers; None for an event.
    pub corr: Option<String>,
    pub resp: Resp,
}

/// One text frame from the core, or None when it is not a frame of the API.
/// An answer whose `resp` has a type this window does not read is still a
/// frame, as `Resp::Other`.
pub fn incoming(text: &str) -> Option<Incoming> {
    let v = parse(text)?;
    let corr = match v.get("corrId") {
        Value::Str(s) => Some(s.clone()),
        Value::Null => None,
        _ => return None,
    };
    Some(Incoming { corr, resp: resp(v.get("resp"))? })
}
