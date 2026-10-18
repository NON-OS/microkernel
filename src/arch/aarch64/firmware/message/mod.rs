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

//! The VideoCore firmware's property message, encoded and read back, as
//! Raspberry Pi's published mailbox property interface documentation gives
//! it (raspberrypi/firmware wiki, "Mailbox property interface").
//!
//! A buffer is: its size in bytes, a code (0 to ask; 0x8000_0000 when the
//! firmware answered, 0x8000_0001 when it could not parse it), the tags, a
//! zero end tag. A tag is: its id, its value buffer's size in bytes, a code
//! (0 to ask; bit 31 set and bits 30:0 the length when answered), and the
//! value buffer, which holds the request and is overwritten by the answer.
//! Pure, so the host proofs run it as it is.

mod answer;
mod encode;
mod tag;

pub(crate) use answer::answer;
pub(crate) use encode::encode;
pub(crate) use tag::{Tag, WORDS};
