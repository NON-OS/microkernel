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

//! A request as a driver receives it.

use crate::errno::{Errno, EINVAL};
use crate::header::{Header, HDR_LEN, MAX_REQUEST};

/// A request as a driver receives it: its header and exactly the
/// payload the header declares, the whole at most 256 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request<'a> {
    pub header: Header,
    pub payload: &'a [u8],
}

impl<'a> Request<'a> {
    pub fn decode(frame: &'a [u8]) -> Result<Self, Errno> {
        let header = Header::decode(frame)?;
        let end = HDR_LEN.checked_add(header.payload_len as usize).ok_or(EINVAL)?;
        if end > MAX_REQUEST || end > frame.len() {
            return Err(EINVAL);
        }
        Ok(Request { header, payload: &frame[HDR_LEN..end] })
    }
}
