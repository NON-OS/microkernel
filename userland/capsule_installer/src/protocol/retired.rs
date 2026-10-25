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

//! The ops the installer retired. 3, OP_LOAD_FROM_STORE, loaded a capsule from
//! bytes in the request as the installer's own child, with no sender to bound
//! it by; nothing sent it, and S-13 removed it. It answers EINVAL, and its
//! number stays taken so no new op reuses it.

use alloc::vec::Vec;

use super::encode::encode_response;
use super::types::{Request, EINVAL, OP_LOAD_FROM_STORE};

pub const RETIRED: [u16; 1] = [OP_LOAD_FROM_STORE];

/// The answer to a retired op, or None for any other.
pub fn retired(req: &Request<'_>) -> Option<Vec<u8>> {
    RETIRED.contains(&req.op).then(|| encode_response(req.seq, EINVAL, &[]))
}
