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

//! The hosts this driver binds, by id. The list is as observed,
//! unverified against each vendor's documents. No UFS host is bound by
//! class alone: each listed host is one whose needs are known. The kernel
//! starts it for any UFS function (class 01h, subclass 09h) and a host not
//! listed here is named in the log and left alone.

pub const INTEL: u16 = 0x8086;

pub const HOSTS: &[(u16, u16)] = &[
    (0x1B36, 0x0013), // QEMU's ufs device
    (0x144D, 0xC00C), // Samsung
    (INTEL, 0x9DFA),  // Cannon Lake
    (INTEL, 0x4B41),  // Elkhart Lake
    (INTEL, 0x4B43),  // Elkhart Lake
    (INTEL, 0x98FA),  // Lakefield
    (INTEL, 0x51FF),  // Alder Lake
    (INTEL, 0x54FF),  // Alder Lake-N
    (INTEL, 0x7E47),  // Meteor Lake
    (INTEL, 0xA847),  // Lunar Lake
    (INTEL, 0x7747),  // Arrow Lake
    (INTEL, 0xE447),  // Panther Lake
];

pub fn is_host(vendor: u16, device: u16) -> bool {
    HOSTS.contains(&(vendor, device))
}

/// The UFS host class: mass storage, UFS.
pub fn is_ufs_class(class: u8, subclass: u8) -> bool {
    class == 0x01 && subclass == 0x09
}
