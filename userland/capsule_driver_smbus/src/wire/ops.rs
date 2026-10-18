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

//! The ops driver.smbus0 answers, and its frame sizes.

use super::header::HDR_LEN;
use crate::xfer::{BLOCK_MAX, HEAD};

pub const OP_HEALTHCHECK: u16 = 1;
/// The host's PCI ids and how many ports its BAR spans: u16 vendor, u16
/// device, u16 ports, u16 reserved.
pub const OP_INFO: u16 = 2;
/// One SMBus transfer (`xfer::parse`). The reply's data is a length byte
/// and the bytes a read returned.
pub const OP_TRANSFER: u16 = 3;

pub const STATUS_LEN: usize = 4;
pub const INFO_LEN: usize = 8;
pub const REQUEST_MAX: usize = HDR_LEN + HEAD + BLOCK_MAX;
pub const REPLY_MAX: usize = HDR_LEN + STATUS_LEN + 1 + BLOCK_MAX;
