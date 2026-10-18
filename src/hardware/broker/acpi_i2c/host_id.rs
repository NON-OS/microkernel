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

//! The host controller an I2C-HID record names, by device_id, pure (K10g):
//! a platform host by its window, a PCI LPSS host by its `_ADR`.
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

/// The device_id of the PCI function the NameSeg `name` resolves to:
/// `hosts` pairs each firmware Device name with its `_ADR`, `functions`
/// each enumerated segment 0 function (bus, device, function) with its
/// device_id. LPSS sits on the root bus. A name declared at two addresses,
/// an `_ADR` naming every function (0xFFFF), or one not enumerated resolves
/// to nothing.
pub(crate) fn pci_host_device_id(
    name: [u8; 4],
    hosts: impl IntoIterator<Item = ([u8; 4], u32)>,
    functions: &[((u8, u8, u8), u64)],
) -> u64 {
    if name == [0; 4] {
        return UNRESOLVED;
    }
    let mut adr = None;
    for (_, a) in hosts.into_iter().filter(|&(n, _)| n == name) {
        if adr.is_some_and(|seen| seen != a) {
            return UNRESOLVED;
        }
        adr = Some(a);
    }
    let Some(adr) = adr else { return UNRESOLVED };
    let (device, function) = (adr >> 16, adr & 0xFFFF);
    if device > 31 || function > 7 {
        return UNRESOLVED;
    }
    let at = (0, device as u8, function as u8);
    functions.iter().find(|&&(bdf, _)| bdf == at).map_or(UNRESOLVED, |&(_, id)| id)
}
