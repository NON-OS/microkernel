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

use alloc::vec::Vec;

use crate::protocol::{encode_response, Request, OP_STORE_STATUS};

pub fn store_status(req: Request<'_>) -> Vec<u8> {
    // The settled word follows the code, so a client reading only the code
    // is unaffected.
    let mut body = [0u8; 8];
    body[..4].copy_from_slice(&crate::blk::status::current().to_le_bytes());
    body[4..].copy_from_slice(&u32::from(crate::blk::status::settled()).to_le_bytes());
    encode_response(OP_STORE_STATUS, req.flags, req.request_id, 0, &body)
}
