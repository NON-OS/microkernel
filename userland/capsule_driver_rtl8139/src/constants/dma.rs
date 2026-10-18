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

pub const RX_BUF_DATA_BYTES: usize = 32 * 1024;
pub const RX_BUF_BYTES: usize = RX_BUF_DATA_BYTES + 16 + 2048;
pub const TX_SLOT_COUNT: usize = 4;
pub const TX_SLOT_BYTES: usize = 2048;
pub const TX_BUF_BYTES: usize = TX_SLOT_COUNT * TX_SLOT_BYTES;

/// Frames in flight this long with the link up and none finished is a hung
/// transmitter: Linux 8139too's watchdog timeout, 6 seconds.
pub const TX_HANG_MS: u64 = 6000;
