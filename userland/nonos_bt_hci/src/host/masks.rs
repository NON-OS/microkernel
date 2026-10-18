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

//! The events an LE-only host asks for, and no others, so a controller
//! does not fill the transport with BR/EDR events nothing here reads.
//! Command Complete, Command Status and Number of Completed Packets come
//! whatever the mask says.

/// Set Event Mask (Vol 4, Part E, 7.3.1): Disconnection Complete (bit 4),
/// Encryption Change (bit 7), Hardware Error (bit 15), Encryption Key
/// Refresh Complete (bit 47), LE Meta (bit 61).
pub const EVENT_MASK: u64 = 1 << 4 | 1 << 7 | 1 << 15 | 1 << 47 | 1 << 61;

/// LE Set Event Mask (Vol 4, Part E, 7.8.1): LE Connection Complete
/// (bit 0), LE Advertising Report (bit 1), LE Connection Update Complete
/// (bit 2), LE Read Remote Features Complete (bit 3), LE Long Term Key
/// Request (bit 4).
pub const LE_EVENT_MASK: u64 = 0x1f;

/// LE Directed Advertising Report (bit 10), asked for only of a controller
/// that scans with the extended filter policies, the only one that sends
/// it (7.7.65.11).
pub const LE_DIRECTED_REPORT: u64 = 1 << 10;

/// Extended Scanner Filter Policies, bit 7 of the LE features (Vol 6,
/// Part B, 4.6) that LE Read Local Supported Features returns (7.8.3).
pub const EXTENDED_SCANNER_FILTER: u64 = 1 << 7;

/// LE Data Packet Length Extension, bit 5 of the LE features (Vol 6,
/// Part B, 4.6).
pub const DATA_LENGTH_EXTENSION: u64 = 1 << 5;
