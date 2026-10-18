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

//! What MkDeviceList shows of a bar, pure. A window's base is a
//! physical address, shown only to the device's holder; a kind-None bar's
//! base carries a name (a `_HID`, a host's device_id), never an address,
//! and is shown to all. Proven on the host (kernel_proofs::device_shown).

/// `BarKind::None`; the kernel asserts the two agree.
pub(crate) const KIND_NONE: u8 = 0;

/// The base the caller reads: zero for a window, or any kind but None,
/// the caller does not hold.
pub(crate) const fn shown_base(base: u64, kind: u8, held: bool) -> u64 {
    if held || kind == KIND_NONE {
        base
    } else {
        0
    }
}
