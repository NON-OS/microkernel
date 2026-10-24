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

//! A frame is `len: u32 | type: u8 | body`, and after the hellos the type
//! and body are sealed, with a 16-byte tag after them. `wire_len` judges the
//! length field before anything after it is read; `split` takes the type
//! and body once the caller has them in plaintext.

use crate::limits::{LEN_BYTES, TAG_LEN};
use crate::{Expect, Kind, WireError};

const fn tag(expect: Expect) -> usize {
    if expect.sealed() {
        TAG_LEN
    } else {
        0
    }
}

/// How many bytes follow the length field, or why the frame is refused.
pub fn wire_len(len: [u8; LEN_BYTES], expect: Expect) -> Result<usize, WireError> {
    let n = usize::try_from(u32::from_le_bytes(len)).map_err(|_| WireError::Oversize)?;
    if n > expect.cap() + tag(expect) {
        return Err(WireError::Oversize);
    }
    if n < 1 + tag(expect) {
        return Err(WireError::Short);
    }
    Ok(n)
}

/// The type byte and body of a frame in plaintext, refused when the type is
/// unknown or may not arrive now.
pub fn split(inner: &[u8], expect: Expect) -> Result<(Kind, &[u8]), WireError> {
    if inner.len() > expect.cap() {
        return Err(WireError::Oversize);
    }
    let (first, body) = inner.split_first().ok_or(WireError::Short)?;
    let kind = Kind::from_byte(*first)?;
    if !expect.admits(kind) {
        return Err(WireError::Order);
    }
    Ok((kind, body))
}

/// The length field for a body of `body_len` bytes sent at `expect`'s
/// stage of the peer, refused when the peer would refuse it.
pub fn len_field(body_len: usize, expect: Expect) -> Result<[u8; LEN_BYTES], WireError> {
    let inner = body_len.checked_add(1).ok_or(WireError::Oversize)?;
    if inner > expect.cap() {
        return Err(WireError::Oversize);
    }
    let n = u32::try_from(inner + tag(expect)).map_err(|_| WireError::Oversize)?;
    Ok(n.to_le_bytes())
}
