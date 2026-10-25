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

//! Whether the body after a request's header is the one the header names.
//! It must be exactly payload_len bytes, so a request cut or padded on the
//! way is refused. Only WRITE carries data; every other op carries at most
//! the READ and WRITE head, and an op with no body answers for the rest.

use crate::header::Request;
use crate::limits::RW_HEADER_LEN;
use crate::ops::OP_WRITE;

pub fn body_fits(req: &Request, body: usize) -> bool {
    body == req.payload_len as usize && (req.op == OP_WRITE || body <= RW_HEADER_LEN)
}
