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

//! The control family's tag, header and ops.

/// The control family tag, distinct from net_core's `MAGIC_NNET`.
pub(super) const WIFI_MAGIC: u32 = 0x5749_4649;

/// Control header: magic, u16 op, u32 request id.
pub(super) const WIFI_HDR: usize = 10;

/// Join a network and associate.
pub(super) const OP_CONNECT: u16 = 1;

/// Tear the session down and clear the keys.
pub(super) const OP_DISCONNECT: u16 = 2;

/// Return the networks the background scanner has collected.
pub(super) const OP_SCAN: u16 = 3;

/// Report how far bring-up got, so the panel can show it on a serial-less machine.
pub(super) const OP_STATUS: u16 = 4;

/// Whether the radio is associated, and with which network.
pub(super) const OP_LINK: u16 = 5;
