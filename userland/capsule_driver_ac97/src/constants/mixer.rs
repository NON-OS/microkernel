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

//! The codec's registers (AC'97 2.3 mixer map) and the values written.

pub const RESET: u8 = 0x00;
pub const MASTER: u8 = 0x02;
pub const HEADPHONE: u8 = 0x04;
pub const PCM_OUT: u8 = 0x18;
pub const POWERDOWN: u8 = 0x26;
pub const EXT_CTRL: u8 = 0x2A;
pub const VENDOR_ID1: u8 = 0x7C;
pub const VENDOR_ID2: u8 = 0x7E;

/// Bit 15 of every volume register.
pub const MUTE: u16 = 1 << 15;
/// Master and headphone: no attenuation. The audio server applies the
/// user's volume to the samples, so the codec stays at full level.
pub const FULL_LEVEL: u16 = 0x0000;
/// PCM out: 0 dB on both channels (gain 8 of 31, where 8 is unity).
pub const PCM_UNITY: u16 = 0x0808;

/// Powerdown status: the DAC, the analog mixer and Vref ready.
pub const PD_PLAYBACK_READY: u16 = (1 << 1) | (1 << 2) | (1 << 3);

/// Extended control: variable rate audio. Left off, the front DAC runs at
/// the link's 48 kHz, which is the rate the audio server mixes at.
pub const EXT_VRA: u16 = 1 << 0;
