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

//! Where each structure sits in the 76 KiB schedule region. One request is
//! served at a time, so control and bulk transfers share their qTDs; each
//! interrupt endpoint has its own, and each polled bulk IN, left armed,
//! holds one of POLL_PLACES. Every QH has a fixed place (places.rs).

use crate::facts::port::PORTS_MAX;
use crate::facts::qh::QH_LEN;
use crate::facts::qtd::QTD_LEN;

/// The periodic frame list: 1024 links, 4 KiB aligned.
pub const FRAME_LIST: usize = 0x0000;
pub const FRAMES: usize = 1024;
/// The QHs: the async ring's head, address 0's control QH, a control QH
/// per slot, two bulk QHs per slot (IN, then OUT), two for each second
/// pair, then the interrupt QHs.
pub const QH_AREA: usize = 0x1000;
pub const SLOTS: usize = 16;
pub const BULK_QHS: usize = 2 * SLOTS;
pub const INTR_MAX: usize = 8;
/// Second bulk pairs (OP_CONFIGURE_BULK2, UAS's data pipes), taken by
/// slots as they come: the QH area holds two and no more.
pub const SECOND_PAIRS: usize = 2;
/// The QHs in the async ring, in ring order.
pub const RING: usize = 2 + SLOTS + BULK_QHS + 2 * SECOND_PAIRS;
/// Control's SETUP, data and STATUS qTDs, bulk's one, each interrupt
/// endpoint's one, each polled place's one.
pub const CTRL_QTD: usize = 0x2000;
pub const BULK_QTD: usize = 0x2080;
pub const INTR_QTD: usize = 0x2100;
pub const POLL_QTD: usize = 0x2200;
pub const SETUP_BUF: usize = 0x2400;
pub const CTRL_BUF: usize = 0x2410;
pub const CTRL_MAX: usize = 512;
pub const INTR_BUF: usize = 0x2800;
pub const INTR_BUF_LEN: usize = 64;
/// Page aligned, so a 4 KiB transfer is one qTD buffer page.
pub const BULK_BUF: usize = 0x3000;
pub const BULK_MAX: usize = 4096;
pub const POLL_BUF: usize = 0x4000;
/// Polled places: one a root port's worth, held by devices, not ports.
pub const POLL_PLACES: usize = PORTS_MAX as usize;
pub const REGION: usize = POLL_BUF + BULK_MAX * POLL_PLACES;

pub use super::places::*;

const _: () = assert!(QH_AREA + QH_LEN * (RING + INTR_MAX) <= CTRL_QTD);
const _: () = assert!(CTRL_QTD + 3 * QTD_LEN <= BULK_QTD && BULK_QTD + QTD_LEN <= INTR_QTD);
const _: () = assert!(INTR_QTD + QTD_LEN * INTR_MAX <= POLL_QTD);
const _: () = assert!(POLL_QTD + QTD_LEN * POLL_PLACES <= SETUP_BUF);
const _: () = assert!(CTRL_BUF + CTRL_MAX <= INTR_BUF);
const _: () = assert!(INTR_BUF + INTR_BUF_LEN * INTR_MAX <= BULK_BUF);
const _: () = assert!(BULK_BUF + BULK_MAX <= POLL_BUF);
