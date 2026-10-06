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

use super::hid_match::id_matches;

/// Length of one community's register window (coreboot GPIO_BASE_SIZE).
const COMMUNITY_LEN: u64 = 0x1_0000;

// coreboot soc/intel/<platform>/include/soc/pcr_ids.h PID_GPIOCOMn, listed
// in the order of the communities in Linux's table for the same `_HID`
// (its gpps names say which COMn each one is). Only platforms whose
// coreboot gpio.asl patches the same communities into `_CRS` are here.
const SPT: &[u8] = &[0xAF, 0xAE, 0xAC];
const CNP_LP: &[u8] = &[0x6E, 0x6D, 0x6A];
const COM_0145: &[u8] = &[0x6E, 0x6D, 0x6A, 0x69];
const ADP_S: &[u8] = &[0x6E, 0x6D, 0x6B, 0x6A, 0x69];

const TABLE: &[(&[u8], &[u8])] = &[
    (b"INT344B", SPT),
    (b"INT3451", SPT),
    (b"INT345D", SPT),
    (b"INT34BB", CNP_LP),
    (b"INT3455", COM_0145),
    (b"INT34C8", COM_0145),
    (b"INT34C5", COM_0145),
    (b"INTC1055", COM_0145),
    (b"INTC1057", COM_0145),
    (b"INTC1056", ADP_S),
    (b"INTC1085", ADP_S),
];

/// Sideband port ids of the communities of the PCH named by `hid`, empty
/// when it is not one whose windows live behind SBREG_BAR.
pub(super) fn community_pids(hid: &[u8; 8]) -> &'static [u8] {
    TABLE.iter().find(|(id, _)| id_matches(hid, id)).map_or(&[], |(_, pids)| pids)
}

/// A community's window: SBREG_BAR plus its port id in bits 23:16, which is
/// coreboot's PCRB(pid) and what vendor firmware writes into `_CRS`.
pub fn community_window(sbreg: u64, pid: u8) -> (u64, u64) {
    (sbreg + (u64::from(pid) << 16), COMMUNITY_LEN)
}

/// SBREG_BAR from the P2SB bridge's BAR0 dwords: a 64-bit memory BAR when
/// its type bits read 10b (PCI 3.0, 6.2.5.1). All ones is a bridge that
/// stayed hidden, zero one firmware never placed.
pub fn sbreg_from_bar0(lo: u32, hi: u32) -> Option<u64> {
    if lo == u32::MAX || lo & 1 != 0 {
        return None;
    }
    let upper = if (lo >> 1) & 3 == 2 { u64::from(hi) << 32 } else { 0 };
    let base = upper | u64::from(lo & !0xF);
    (base != 0).then_some(base)
}
