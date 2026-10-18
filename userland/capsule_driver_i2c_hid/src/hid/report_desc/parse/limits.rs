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

//! The hard bounds on a report descriptor walk, under the names usb_hid's
//! walker files reach for. A descriptor past any of them is refused whole,
//! and the device it came from is not driven.

/// A descriptor the walk refuses: past a bound, cut inside an item, or with
/// a collection left open.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refused;

/// Items walked: one per byte of the longest descriptor setup reads, so no
/// descriptor it accepts is cut short by this bound.
pub const MAX_ITEMS: usize = 1024;
/// Collections open at once.
pub const MAX_DEPTH: u8 = 8;
/// Global states saved by Push and not yet restored.
pub const MAX_PUSH: usize = 4;
/// Usages kept for one main item; fields past them take the last one's.
pub const MAX_USAGES: usize = 32;
/// Input reports, and apart from them feature reports, by report ID.
pub const MAX_REPORTS: usize = 16;
/// The widest field a Report Size may give. Only fields of 32 bits or fewer
/// are ever read (field_at.rs).
pub const MAX_FIELD_BITS: u32 = 256;
/// One main item's Report Count, its report size times count, and one
/// report's bits: 16 KiB, generous enough that a vendor's firmware blob or
/// raw-data report is no reason to refuse a pad.
pub const MAX_REPORT_BITS: u32 = 16384 * 8;
/// The longest report in bytes, its ID byte included.
pub const MAX_REPORT_BYTES: u16 = 16384;
/// The longest input report, its ID byte included, that one 256-byte read
/// of the input register holds after the two-byte length (state::FRAME_MAX).
/// A longer one is never read whole, so no layout is taken from it (long.rs).
pub const MAX_INPUT_BYTES: u16 = 254;
