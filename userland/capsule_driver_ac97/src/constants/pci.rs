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

//! The function this driver takes: the 82801AA (ICH) AC'97 audio controller
//! that QEMU's `AC97` and VirtualBox's AC'97 both present. The later ICH
//! parts share the register layout but add 20-bit and six channel modes and
//! memory windows this driver has no model of, so they are left alone.

pub const INTEL_VENDOR_ID: u16 = 0x8086;
pub const ICH_AC97_DEVICE_ID: u16 = 0x2415;

/// Native Audio Mixer: the codec's registers, sixteen bits each.
pub const NAM_BAR: u8 = 0;
/// Native Audio Bus Master: the DMA engines and the link control.
pub const NABM_BAR: u8 = 1;

/// The highest register each window is read at, plus its width: the
/// codec's vendor id at 0x7E and the codec semaphore at 0x34.
pub const NAM_MIN_PORTS: u16 = 0x80;
pub const NABM_MIN_PORTS: u16 = 0x38;
