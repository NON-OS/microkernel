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

//! Where each structure sits in the 12 KiB schedule region. Each ED owns
//! two TD places used in turn: one holds the TDs to run, the other the
//! empty TD its TailP names; they swap when a run ends (sched/queue.rs).

use crate::facts::ed::ED_LEN;
use crate::facts::td::TD_LEN;

/// The HCCA: interrupt table, frame number, done head; 256-byte aligned.
pub const HCCA: usize = 0x000;
pub const HCCA_LEN: usize = 256;
/// The control ED, then one ED per interrupt endpoint, then two bulk EDs
/// per slot (IN, then OUT).
pub const CTRL_ED: usize = 0x100;
pub const INTR_ED: usize = 0x110;
pub const INTR_MAX: usize = 8;
pub const BULK_ED: usize = 0x200;
pub const BULK_EDS: usize = 32;
/// Control's two places of three TDs (SETUP, data, STATUS) and the empty.
pub const CTRL_TD: usize = 0x400;
pub const CTRL_HALF: usize = 4 * TD_LEN;
/// Two TD places per interrupt ED, and per bulk ED.
pub const INTR_TD: usize = 0x480;
pub const BULK_TD: usize = 0x600;
pub const SETUP_BUF: usize = 0xA00;
pub const CTRL_BUF: usize = 0xA10;
pub const CTRL_MAX: usize = 512;
pub const INTR_BUF: usize = 0xC40;
pub const INTR_BUF_LEN: usize = 64;
/// Page aligned, so a 4 KiB TD never crosses a page.
pub const BULK_BUF: usize = 0x1000;
pub const BULK_MAX: usize = 4096;
pub const POLL_BUF: usize = 0x2000;
pub const REGION: usize = 0x3000;

pub const fn intr_ed(i: usize) -> usize {
    INTR_ED + ED_LEN * i
}
pub const fn intr_td(i: usize, half: u8) -> usize {
    INTR_TD + TD_LEN * (2 * i + half as usize)
}
pub const fn intr_buf(i: usize) -> usize {
    INTR_BUF + INTR_BUF_LEN * i
}
pub const fn ctrl_td(half: u8) -> usize {
    CTRL_TD + CTRL_HALF * half as usize
}
/// Bulk ED `i`: slot `i / 2 + 1`, IN when `i` is even.
pub const fn bulk_ed(i: usize) -> usize {
    BULK_ED + ED_LEN * i
}
pub const fn bulk_td(i: usize, half: u8) -> usize {
    BULK_TD + TD_LEN * (2 * i + half as usize)
}

const _: () = assert!(intr_ed(INTR_MAX) <= BULK_ED && bulk_ed(BULK_EDS) <= CTRL_TD);
const _: () = assert!(ctrl_td(2) <= INTR_TD && intr_td(INTR_MAX, 0) <= BULK_TD);
const _: () = assert!(bulk_td(BULK_EDS, 0) <= SETUP_BUF);
const _: () = assert!(CTRL_BUF + CTRL_MAX <= INTR_BUF && intr_buf(INTR_MAX) <= BULK_BUF);
const _: () = assert!(BULK_BUF + BULK_MAX <= POLL_BUF && POLL_BUF + BULK_MAX <= REGION);
