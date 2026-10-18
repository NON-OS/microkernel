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
//! What differs between the controllers this driver meets, by PCI identity,
//! from the device table in sound/pci/hda/hda_intel.c.
//!
//! - Intel from Skylake on (`AZX_DRIVER_SKL`, Apollo Lake and Gemini Lake
//!   among them) carries an audio DSP, may report PCI class 0x0401 when
//!   firmware enabled it, has a multi-link capability whose link clock Linux
//!   checks after reset (`intel_init_lctl`), and is read by the DMA position
//!   buffer for playback (`POS_FIX_SKL`).
//! - Apollo Lake (8086:5a98) alone has its DMA FIFO threshold lowered
//!   (`bxt_reduce_dma_latency`); Gemini Lake (8086:3198) does not.
//! - AMD's chipset controllers (1022:15e3, 1457, 1487) and every ATI, AMD or
//!   NVIDIA graphics controller are read by LPIB (`AZX_DCAPS_POSFIX_LPIB`,
//!   `POS_FIX_FIFO`); a graphics card's controller carries HDMI only.
//! - AMD's audio coprocessor (1022:15e2, class 0x0480) is not an HD Audio
//!   controller at all.

mod identity;
mod latency;
mod link_cap;
mod link_clock;
mod quirks;

pub use identity::*;
pub use latency::*;
pub use link_cap::*;
pub use link_clock::*;
pub use quirks::*;
