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

//! Op discriminants for `driver.igb_0`, the NNET set every wired card
//! serves. The counters are on 7: net.core asks every card for OP_RX_BATCH
//! (6) first and takes an empty batch as the answer of a driver that serves
//! batches, which this one does.

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_LINK_STATUS: u16 = 2;
pub const OP_MAC_ADDRESS: u16 = 3;
pub const OP_TX_PACKET: u16 = 4;
pub const OP_RX_PACKET: u16 = 5;
/// Every waiting frame in one numbered reply, as e1000, igc and virtio_net serve it.
pub const OP_RX_BATCH: u16 = 6;
pub const OP_STATS: u16 = 7;
