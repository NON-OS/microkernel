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

//! The head every LEGACY reply starts with: count, 8042 verdict, boot flags.

use super::find::Legacies;
use alloc::vec::Vec;
use nonos_aml::legacy::BootArch;

fn flags(b: &BootArch) -> u16 {
    let bits = [b.legacy_devices, b.has_8042, b.no_vga, b.no_msi, b.no_aspm, b.no_cmos_rtc];
    bits.iter().enumerate().fold(0, |v, (i, on)| v | (u16::from(*on) << i))
}

pub(super) fn head(l: &Legacies) -> Vec<u8> {
    let mut v = (l.list.len() as u32).to_le_bytes().to_vec();
    v.push(u8::from(l.has_8042));
    v.extend_from_slice(&l.boot.as_ref().map_or(0xFFFF, flags).to_le_bytes());
    v
}
