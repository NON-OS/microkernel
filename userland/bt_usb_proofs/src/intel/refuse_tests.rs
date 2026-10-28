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

use super::model::Intel;
use super::name_tests::BZRI;
use super::rig::{bootloader, run, tlv, TYP};
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode::Opcode;

#[test]
fn firmware_the_bootloader_rejects_is_not_booted_and_the_part_is_sent_back() {
    let mut m = bootloader(TYP, 1);
    m.result = 0x01;
    let (r, m, _) = run(m, 0x0032);
    assert_eq!(r, Err(HciError::Status(Opcode(0xfc09), 0x01)));
    assert_eq!((m.boot_addr, m.booted, m.to_bootloader), (None, false, 1));
    assert_eq!(m.ops.last(), Some(&0xfc01));
}

#[test]
fn a_refused_fragment_ends_the_download_there() {
    let mut m = bootloader(TYP, 1);
    m.refuse_fragment = 4;
    let (r, m, _) = run(m, 0x0032);
    assert_eq!(r, Err(HciError::Status(Opcode(0xfc09), 0x12)));
    assert_eq!((m.fragments.len(), m.to_bootloader, m.boot_addr), (4, 1, None));
}

#[test]
fn firmware_the_tree_does_not_carry_is_refused_by_name_before_any_is_sent() {
    let m = Intel { tlv: [tlv(BZRI, 0x01, [0; 3], 1), Vec::new()], ..Intel::default() };
    let (r, m, said) = run(m, 0x0035);
    assert_eq!(r, Err(HciError::NotBluetooth));
    assert_eq!(m.ops, [0xfc05]);
    assert_eq!(said, ["intel: firmware not carried: ibt-0190-0291-iml.sfi"]);
}

#[test]
fn another_platform_or_an_unknown_variant_is_refused() {
    let mut other = tlv(TYP, 0x01, [0; 3], 1);
    assert_eq!(other[15], 0x37, "the platform octet of CNVi BT");
    other[15] = 0x38;
    let m = Intel { tlv: [other, Vec::new()], ..Intel::default() };
    assert_eq!(run(m, 0x0032).2, ["intel: not an Intel Bluetooth platform"]);
    let m = Intel { tlv: [tlv((0, 0, 0x1a, ""), 0x01, [0; 3], 1), Vec::new()], ..Intel::default() };
    let (r, m, said) = run(m, 0x0032);
    assert_eq!((r, m.ops.len()), (Err(HciError::NotBluetooth), 1));
    assert_eq!(said, ["intel: hardware variant not supported"]);
}
