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

//! Where each structure sits in the 20 KiB schedule region. One request is
//! served at a time, so each kind of transfer has one fixed place.

use crate::facts::qh::QH_LEN;
use crate::facts::td::TD_LEN;

/// The frame list: 1024 links, one a frame.
pub const FRAME_LIST: usize = 0x0000;
pub const FRAMES: usize = 1024;
/// A queue head per interrupt endpoint, then control's, bulk's, and the
/// polled bulk IN's, which stays armed and so has a queue of its own.
pub const INTR_QH: usize = 0x1000;
pub const INTR_MAX: usize = 8;
pub const CTRL_QH: usize = INTR_QH + QH_LEN * INTR_MAX;
pub const BULK_QH: usize = CTRL_QH + QH_LEN;
pub const POLL_QH: usize = BULK_QH + QH_LEN;
/// One armed TD per interrupt endpoint.
pub const INTR_TD: usize = 0x1100;
/// A control transfer: SETUP, at most 64 data TDs (512 bytes at a low-speed
/// device's 8-byte packets), STATUS.
pub const CTRL_TD: usize = 0x1200;
pub const CTRL_TDS: usize = 66;
/// A bulk transfer: 4 KiB at 64-byte packets.
pub const BULK_TD: usize = 0x1800;
pub const BULK_TDS: usize = 64;
/// The polled bulk IN's chain.
pub const POLL_TD: usize = 0x1C00;
/// The SETUP packet, then control's data.
pub const SETUP_BUF: usize = 0x2000;
pub const CTRL_BUF: usize = 0x2010;
pub const CTRL_MAX: usize = 512;
/// One report buffer per interrupt endpoint.
pub const INTR_BUF: usize = 0x2400;
pub const INTR_BUF_LEN: usize = 64;
pub const BULK_BUF: usize = 0x3000;
pub const BULK_MAX: usize = 4096;
pub const POLL_BUF: usize = 0x4000;
pub const REGION: usize = 0x5000;

/// Interrupt endpoint `i`'s queue, TD and report buffer.
pub const fn intr_qh(i: usize) -> usize {
    INTR_QH + QH_LEN * i
}
pub const fn intr_td(i: usize) -> usize {
    INTR_TD + TD_LEN * i
}
pub const fn intr_buf(i: usize) -> usize {
    INTR_BUF + INTR_BUF_LEN * i
}

const _: () = assert!(INTR_TD + TD_LEN * INTR_MAX <= CTRL_TD);
const _: () = assert!(CTRL_TD + TD_LEN * CTRL_TDS <= BULK_TD);
const _: () = assert!(BULK_TD + TD_LEN * BULK_TDS <= POLL_TD);
const _: () = assert!(POLL_TD + TD_LEN * BULK_TDS <= SETUP_BUF);
const _: () = assert!(CTRL_BUF + CTRL_MAX <= INTR_BUF);
const _: () = assert!(INTR_BUF + INTR_BUF_LEN * INTR_MAX <= BULK_BUF);
const _: () = assert!(BULK_BUF + BULK_MAX <= POLL_BUF && POLL_BUF + BULK_MAX <= REGION);
