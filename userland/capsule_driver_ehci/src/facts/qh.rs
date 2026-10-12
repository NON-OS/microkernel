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

//! The queue head: twelve 32-bit words, 32-byte aligned (64 bytes kept).
//!
//! - Horizontal link: the next QH, type QH (01) in 2:1, terminate bit 0.
//! - Characteristics: address 6:0, endpoint 11:8, speed 13:12 (2: high),
//!   data toggle from the qTD 14 (DTC), head of reclamation 15 (H),
//!   maximum packet 26:16, control endpoint 27 (C: a full- or low-speed
//!   control endpoint only), NAK reload 31:28.
//! - Capabilities: interrupt schedule mask 7:0 and split completion mask
//!   15:8 (periodic only), the TT's hub address 22:16 and port 29:23 (full
//!   and low speed only), high-bandwidth multiplier 31:30 (1).
//!
//! (EHCI 1.0 section 3.6, as recalled, unverified.)
//! - Current qTD, then the overlay: next qTD, alternate next qTD, token
//!   and buffer pointers, as a qTD's.

pub const QH_LEN: usize = 64;
pub const LINK_T: u32 = 1 << 0;
pub const LINK_QH: u32 = 1 << 1;
pub const CH_EP_SHIFT: u32 = 8;
pub const CH_EPS_FULL: u32 = 0;
pub const CH_EPS_LOW: u32 = 1 << 12;
pub const CH_EPS_HIGH: u32 = 2 << 12;
pub const CH_EPS_MASK: u32 = 3 << 12;
pub const CH_C: u32 = 1 << 27;
pub const CH_DTC: u32 = 1 << 14;
pub const CH_H: u32 = 1 << 15;
pub const CH_MPS_SHIFT: u32 = 16;
pub const CH_RL: u32 = 4 << 28;
pub const CAP_SMASK_UFRAME0: u32 = 0x01;
/// Complete-splits in micro-frames 2, 3 and 4, after a start-split in 0.
pub const CAP_CMASK_SPLIT: u32 = 0x1C << 8;
pub const CAP_HUB_SHIFT: u32 = 16;
pub const CAP_PORT_SHIFT: u32 = 23;
pub const CAP_MULT_1: u32 = 1 << 30;
/// Word offsets of the current qTD and the overlay.
pub const QH_CURRENT: usize = 12;
pub const QH_NEXT: usize = 16;
pub const QH_ALT: usize = 20;
pub const QH_TOKEN: usize = 24;
