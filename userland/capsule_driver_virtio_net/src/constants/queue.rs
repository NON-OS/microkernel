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

pub const Q_RX: u16 = 0;
pub const Q_TX: u16 = 1;

pub const RX_QUEUE_SIZE: u16 = 64;
// Frames in flight at once. net.core drops a frame answered E_AGAIN, so this
// holds a burst as the wired drivers' 32-entry rings do; one in-flight bit
// each (queue::TxQueue), and the first 32 descriptors of the 256-entry ring.
pub const TX_QUEUE_SIZE: u16 = 32;
pub const VRING_DESC_F_WRITE: u16 = 2;

pub const VQ_DESC_OFFSET: usize = 0;
pub const VQ_AVAIL_OFFSET: usize = 4096;
pub const VQ_USED_OFFSET: usize = 8192;
pub const VQ_REGION_SIZE: usize = 12288;

// Physical virtqueue slot count. The offsets above hold only for a 256-entry
// ring (256 descriptors * 16 bytes = the avail offset), so it is the only size
// either transport accepts: a legacy device keeps its QueueNumMax and lays its
// rings out for that from the page number, and at 512 or more its used ring
// starts at or past the end of the grant. Avail and used ring positions index
// modulo THIS, never the smaller number of buffers the driver primes.
pub const RING_SLOTS: u16 = (VQ_AVAIL_OFFSET / 16) as u16;

pub const RX_DESC_COUNT: u16 = RX_QUEUE_SIZE;
pub const TX_DESC_COUNT: u16 = TX_QUEUE_SIZE;

pub const RX_BUFFER_LEN: u32 = 2048;
pub const TX_BUFFER_LEN: u32 = 2048;

// The layout the grant holds, checked where it is defined: one in-flight bit
// per transmit buffer, and both rings of a 256-entry queue (event words
// included) inside the 12 KiB region.
const _: () = assert!(TX_QUEUE_SIZE as u32 <= u32::BITS && TX_QUEUE_SIZE <= RING_SLOTS);
const _: () = assert!(VQ_AVAIL_OFFSET + 6 + 2 * RING_SLOTS as usize <= VQ_USED_OFFSET);
const _: () = assert!(VQ_USED_OFFSET + 6 + 8 * RING_SLOTS as usize <= VQ_REGION_SIZE);
