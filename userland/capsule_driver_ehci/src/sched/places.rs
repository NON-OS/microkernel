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

//! Each QH's, qTD's and buffer's place in the layout. The async ring is
//! the QHs 0 to RING - 1 in order, the last linked back to the first.

use super::layout::*;
use crate::facts::qh::QH_LEN;
use crate::facts::qtd::QTD_LEN;

/// The QH at ring index `i` (interrupt QHs follow the ring).
pub const fn qh(i: usize) -> usize {
    QH_AREA + QH_LEN * i
}
/// The QH before ring index `i`, whose link names it.
pub const fn ring_prev(i: usize) -> usize {
    qh(if i == 0 { RING - 1 } else { i - 1 })
}
pub const ASYNC_HEAD: usize = 0;
pub const ADDR0_RING: usize = 1;
/// Ring indices: slot `s`'s control QH, and its bulk QHs (IN, then OUT).
pub const fn ctrl_ring(slot: u8) -> usize {
    1 + slot as usize
}
pub const fn bulk_ring(slot: u8, input: bool) -> usize {
    2 + SLOTS + 2 * (slot as usize - 1) + if input { 0 } else { 1 }
}
/// Interrupt endpoint `i`'s QH, qTD and buffer.
pub const fn intr_qh(i: usize) -> usize {
    qh(RING + i)
}
pub const fn intr_qtd(i: usize) -> usize {
    INTR_QTD + QTD_LEN * i
}
pub const fn intr_buf(i: usize) -> usize {
    INTR_BUF + INTR_BUF_LEN * i
}
/// Root port `lane + 1`'s polled bulk IN qTD and buffer.
pub const fn poll_qtd(lane: usize) -> usize {
    POLL_QTD + QTD_LEN * lane
}
pub const fn poll_buf(lane: usize) -> usize {
    POLL_BUF + BULK_MAX * lane
}
