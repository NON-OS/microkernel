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

//! The host controller an I2C-HID record names, by device_id, pure (K10g).
//! The record once carried the host's physical base, which every caller
//! of MkDeviceList could read. Proven on the host (kernel_proofs::device_shown).

/// No registered host matched the record's controller.
pub(crate) const UNRESOLVED: u64 = u64::MAX;

/// The device_id of the registered host the NameSeg `name` resolves to:
/// `hosts` pairs each firmware host name with its MMIO base, `registered`
/// each registered host record's MMIO base with its device_id. A zero
/// name or base resolves to nothing.
pub(crate) fn host_device_id(
    name: [u8; 4],
    hosts: impl IntoIterator<Item = ([u8; 4], u64)>,
    registered: &[(u64, u64)],
) -> u64 {
    if name == [0; 4] {
        return UNRESOLVED;
    }
    let Some((_, base)) = hosts.into_iter().find(|&(n, _)| n == name) else {
        return UNRESOLVED;
    };
    if base == 0 {
        return UNRESOLVED;
    }
    registered.iter().find(|&&(b, _)| b == base).map_or(UNRESOLVED, |&(_, id)| id)
}
