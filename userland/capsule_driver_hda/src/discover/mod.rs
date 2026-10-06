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
//! Finding every HD Audio controller, and the audio hardware that is not one.
//!
//! Only PCI class 0x0403 used to be accepted, and only the first such
//! function. Two kinds of machine fell through that:
//!
//! - Intel laptops from Skylake on whose firmware enabled the audio DSP
//!   report the same controller as class 0x0401. It still runs as a plain HD
//!   Audio controller (Linux `snd_hda_intel` binds it by device id), but the
//!   driver never saw it and the machine read as having no audio.
//! - Desktops and AMD laptops have two or more controllers: the graphics
//!   card's (HDMI only) and the chipset's (the speakers and jacks). The first
//!   in bus order is often the graphics one.
//!
//! So every candidate is collected, chipset controllers before graphics ones,
//! and the AMD audio coprocessor (class 0x0480) and Intel's SST engines
//! (`controller::sst`) are noted so a machine whose sound runs through one
//! is named rather than reported empty or given up on.

mod candidate;
mod found;
mod machine;
mod survey;

pub use found::Found;
pub use machine::Survey;
pub use survey::survey;
