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

//! The flush, and the request ceiling the read and write paths split at.
//! Callers hand in whole sectors; a slice above the per-request ceiling is
//! split in `read.rs` and `write.rs`, so the disk writer above never has
//! to know what the ceiling is.

use super::handle::BlockDevice;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

impl BlockDevice {
    pub fn flush(&self) -> Result<(), BlkError> {
        let op = self.driver.ops().flush;
        let mut rx = [0u8; HDR_LEN + STATUS_LEN];
        let (n, id) = call(self.port, self.driver.magic(), op, &[], &mut rx)?;
        decode_reply(&rx, n, self.driver.magic(), op, id).map(|_| ())
    }
}
