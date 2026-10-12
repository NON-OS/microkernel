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

//! USB Audio 1.0 and 2.0 playback, pure: what a device offers and how its stream is
//! timed. The proofs (`userland/usb_audio_proofs`) run it on descriptors.

mod alt;
mod clock;
mod entities;
mod format;
mod pick;
mod playback;
mod read;
mod request;
mod terminal;
mod timing;

pub use alt::Stream;
pub use clock::clock;
pub use format::RATE_HZ;
pub use playback::playback;
pub use request::{get_clock, select_first, set_clock, set_rate};
pub use terminal::{outputs, Outputs};
pub use timing::{timing, Timing};
