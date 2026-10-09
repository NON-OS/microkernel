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

//! Where IDENTIFY DEVICE keeps the words the driver reads (ACS-3, 7.12.7).

/// IDENTIFY DEVICE answers with one 512-byte block of 256 little-endian words.
pub const IDENTIFY_WORDS: usize = 256;
/// Words 10-19: the serial number, 20 ATA string bytes.
pub const W_SERIAL: usize = 10;
pub const SERIAL_BYTES: usize = 20;
/// Words 27-46: the model number, 40 ATA string bytes.
pub const W_MODEL: usize = 27;
pub const MODEL_BYTES: usize = 40;
/// Word 83: command sets supported. Bit 10 is the 48-bit Address feature set.
pub const W_SUPPORTED_2: usize = 83;
/// Word 86: command sets enabled. Bit 10 is the 48-bit Address feature set.
pub const W_ENABLED_2: usize = 86;
/// Word 87: its validity bits vouch for words 85-87, word 86 among them.
pub const W_ENABLED_DEFAULT: usize = 87;
/// Words 100-103: sectors addressable with 48-bit commands, low word first.
pub const W_LBA48_CAPACITY: usize = 100;
/// Word 106: physical and logical sector size, under its own validity bits.
pub const W_SECTOR_SIZE: usize = 106;
/// Words 117-118: the logical sector length in words, low word first, when
/// word 106 says the sector is longer than 256 words.
pub const W_LOGICAL_SECTOR_WORDS: usize = 117;

/// Bit 10 of words 83 and 86: the 48-bit Address feature set.
pub const FEATURE_LBA48: u16 = 1 << 10;
/// Bit 12 of word 106: the logical sector is longer than 256 words.
pub const SECTOR_LONGER_THAN_256_WORDS: u16 = 1 << 12;
/// Bits 15:14 of words 83, 87 and 106. Such a word holds data only when they
/// read 01b; 11b is what an all-ones block, or a bus nothing drives, gives.
pub const WORD_VALID_MASK: u16 = 0xc000;
pub const WORD_VALID: u16 = 0x4000;
