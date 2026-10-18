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

//! The one coherent page the part and the driver share: both descriptor
//! rings, back to back as DSL 0 wants them, 16-byte aligned.

use super::queue::{DESC_BYTES, RX_DESC_COUNT, TX_DESC_COUNT};

pub const RX_RING_OFF: usize = 0x000;
pub const TX_RING_OFF: usize = RX_RING_OFF + RX_DESC_COUNT * DESC_BYTES;
pub const CONTROL_BYTES: usize = 4096;

const _: () = assert!(TX_RING_OFF + TX_DESC_COUNT * DESC_BYTES <= CONTROL_BYTES);
const _: () = assert!(RX_RING_OFF & 15 == 0 && TX_RING_OFF & 15 == 0);
