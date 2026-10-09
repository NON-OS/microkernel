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

//! A syscall register narrowed to the field it carries, with no kernel
//! dependencies so a host proof can run it. A value the field cannot hold is
//! refused, never cut down: `as u32` on 2^32 + n is n, a pid, port or register
//! the caller never named, and the call then acts on that one instead.

/// `raw` as a 32-bit field (a pid, a port, an endpoint, a flags word).
pub(crate) fn u32_arg(raw: u64) -> Option<u32> {
    u32::try_from(raw).ok()
}

/// `raw` as a 16-bit field (a PCI config register value).
pub(crate) fn u16_arg(raw: u64) -> Option<u16> {
    u16::try_from(raw).ok()
}

/// `raw` as an 8-bit field (a BAR index).
pub(crate) fn u8_arg(raw: u64) -> Option<u8> {
    u8::try_from(raw).ok()
}
