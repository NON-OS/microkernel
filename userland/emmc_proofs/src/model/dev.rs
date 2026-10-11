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

//! What the model host asks of the card in its slot, an eMMC device or an
//! SD memory card: each answers for its own protocol.

use super::card::Out;

pub trait Dev {
    fn power(&mut self, on: bool);
    /// One command, with the host's Block Count register for an open-ended
    /// multi-block transfer.
    fn command(&mut self, idx: u8, arg: u32, host_blocks: u16, now: u64) -> Out;
    /// The data phase is over (all blocks moved, or CMD12).
    fn end(&mut self, now: u64);
    fn take_write(&mut self, lba: u64, data: &[u8], now: u64);
    /// The fastest SD clock the card takes in the state it is in.
    fn max_hz(&self) -> u32;
    /// The count a SET_BLOCK_COUNT left for the next transfer.
    fn count(&self) -> Option<u16>;
    fn width(&self) -> u8;
    /// Data read over `host_width` lines arrives corrupted.
    fn corrupts(&self, host_width: u8) -> bool;
    /// The Command register flags `idx` needs now, or None when a driver
    /// should not send it.
    fn flags(&self, idx: u8) -> Option<u16>;
    /// `idx` has a data phase.
    fn carries_data(&self, idx: u8) -> bool;
}
