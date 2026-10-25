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
//! The microphone wire, NMIC version 1: the host wire's 20-byte header
//! with its own magic, then the body; a reply's payload starts with its
//! status. Samples are 48 kHz signed 16-bit little-endian, the channels
//! of a frame interleaved.

pub const MAGIC: u32 = 0x4E4D_4943; // "NMIC"
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_HEALTHCHECK: u16 = 1;
/// `vendor u16, product u16, channels u8, 0, 0, 0, rate u32`.
pub const OP_OPEN: u16 = 2;
/// The stream started: `channels u8, 0, 0, 0, rate u32`.
pub const OP_START: u16 = 3;
/// `most u16`: `overrun u32`, the bytes lost since the last read because
/// none read them, then up to `most` bytes of whole frames.
pub const OP_READ: u16 = 4;
pub const OP_STOP: u16 = 5;

pub const CHUNK: usize = 4000;
pub const REPLY_MAX: usize = HDR_LEN + 4 + 4 + CHUNK;

/// The caller lacks MicrophoneCapture, or the indicator is not shown.
pub const E_PERM: i32 = -1;
pub const E_BUSY: i32 = -16;
pub const E_NODEV: i32 = -19;
pub const E_INVAL: i32 = -22;
