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

//! Three DMA grants on the channel's record, each at most a network
//! device's ceiling of 64 pages a grant (broker dma/limits.rs): the two
//! rings, 128 KiB each; the receive buffer, which the host fills; the send
//! buffer, which the driver fills.

use nonos_vmbus::Layout;

pub const PAGE: usize = 4096;
pub const RINGS: Layout = Layout { out_pages: 32, in_pages: 32 };
pub const RING_BYTES: u64 = 64 * PAGE as u64;
pub const RECV_BYTES: u64 = 64 * PAGE as u64;
pub const SEND_BYTES: u64 = 64 * PAGE as u64;
/// The largest receive buffer section the host may name: no more than the
/// buffer holds, so a range past it is the host's error, never read.
pub const RECV_PAGES: usize = 64;
pub const SEND_PAGES: usize = 64;
/// The send buffer's sections the driver tracks: one bit each.
pub const MAX_SECTIONS: usize = 64;

const _: () = assert!(RINGS.out_pages as u64 + RINGS.in_pages as u64 == RING_BYTES / PAGE as u64);
const _: () = assert!(RECV_PAGES as u64 * PAGE as u64 == RECV_BYTES);
const _: () = assert!(SEND_PAGES as u64 * PAGE as u64 == SEND_BYTES);
