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

//! Whether the IOC's request frames can carry this driver's requests. The
//! IOC fetches a whole frame of its own size from the request address, so
//! its frame may be larger than the driver's 128 bytes as long as that read
//! stays in the request region of the grant, which holds nothing else; the
//! message and its SGE say how much of it counts. The emulated SAS1068 reports
//! 128 words (512 bytes); the SCSI request with its one SGE must still fit.

use crate::constants::layout::{REPLY_OFF, REQUEST_OFF};
use crate::constants::msg::IO_SGL;

/// The SCSI request header and one 8-byte SGE.
pub const LEAST: usize = IO_SGL + 8;
/// The largest frame whose fetch stays short of the reply frames.
pub const MOST: usize = REPLY_OFF - REQUEST_OFF;

/// `dwords` is the IOC facts' RequestFrameSize, in 32-bit words.
pub fn served(dwords: u16) -> bool {
    (LEAST..=MOST).contains(&(dwords as usize * 4))
}
