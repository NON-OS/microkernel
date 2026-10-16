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
//! Codecs as their parameters describe them, for `sim` to answer with and for
//! the pure planner to read.
//!
//! The ALC236 is the codec in the HP Laptop 15s-fq0xxx (board 8651, Gemini
//! Lake Refresh) this driver is first meant to play on, from the layout
//! Linux codec dumps of that part show: two DACs (0x02, 0x03), no output
//! mixer, the speaker at 0x14 and the headphone jack at 0x21 each choosing
//! between the DACs, a digital microphone at 0x12 and S/PDIF at 0x1e. The
//! Intel display codec carries only HDMI and DisplayPort pins. The QEMU
//! duplex codec is what the driver was first tested on.

mod codecs;
mod desc;
mod params;
mod routing;
mod widget;

pub use codecs::*;
pub use desc::*;
pub use params::*;
pub use routing::*;
pub use widget::*;
