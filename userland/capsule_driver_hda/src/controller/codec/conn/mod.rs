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

//! Connection lists (HDA 1.0a section 7.3.3.3).
//!
//! The driver never read one: it took the first DAC and the first pin it met
//! and hoped they were wired together. They usually are not. On a Realtek
//! ALC236 the first pin is 0x12, a digital microphone input, and the
//! speaker at 0x14 reaches its DAC only through a connection the codec
//! describes here.
//!
//! The list comes back four short entries (8 bits) or two long ones (16
//! bits) per GET_CONNECT_LIST, by the long-form bit 7 of the length
//! parameter. An entry with its top bit set is a range: every node from the
//! entry before it up to this one is connected (Linux
//! `snd_hda_get_raw_connections`). Connection indices, the ones
//! SET_CONNECT_SEL and the input amps take, count the expanded list.

mod list;

pub use list::*;
