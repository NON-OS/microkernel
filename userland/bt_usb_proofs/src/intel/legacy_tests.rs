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

use super::loader::carried::carried;
use super::loader::ddc::entries;
use super::model::Intel;
use super::rig::{ops_after, run, tlv};
use nonos_bt_hci::HciError;

/// An AX201 whose Read Version names ibt-19-<hw>-<fw>, in its bootloader.
fn ax201(hw: u8, fw: u8) -> Intel {
    let old = |fv: u8, b: [u8; 3]| vec![0x37, 0x13, hw, fv, fw, b[0], b[1], b[2], 0];
    let both = [old(0x06, [0; 3]), old(0x23, [0x53, 0x07, 0x1a])];
    let expect = carried("ibt-19-0-4").unwrap().sfi.len() - 644;
    let p = vec![0u8; 22];
    Intel { tlv: both.clone(), old: both, boot_params: p, boots: true, expect, ..Intel::default() }
}

#[test]
fn an_ax201_answers_in_the_old_form_and_takes_ibt_19_by_its_link() {
    let (r, m, said) = run(ax201(32, 1), 0x0026);
    assert_eq!(r, Ok(()));
    assert_eq!(m.ops[..3], [0xfc05, 0xfc0d, 0xfc09]);
    let c = carried("ibt-19-0-4").unwrap();
    let sent: Vec<u8> = m.fragments.iter().flat_map(|f| f[1..].to_vec()).collect();
    assert_eq!(sent, [&c.sfi[..384], &c.sfi[388..]].concat(), "RSA header, then 644 on");
    assert_eq!(m.boot_addr, Some(0x0002_4800));
    assert_eq!(m.ddc, entries(c.ddc).unwrap());
    assert_eq!(ops_after(&m, 0xfc01), [0xfc8b, 0xfc8b, 0xfc52, 0x0c03]);
    assert_eq!(said, ["intel: sending ibt-19-32-1", "intel: booted, DDC from ibt-19-32-1"]);
}

#[test]
fn an_hrp_answering_in_tlv_is_asked_again_in_the_old_form() {
    let mut m = ax201(0, 4);
    m.tlv[0] = tlv((0, 0, 0x13, ""), 0x01, [0; 3], 0);
    let (r, m, _) = run(m, 0x0026);
    assert_eq!(r, Ok(()));
    assert_eq!(m.ops[..3], [0xfc05, 0xfc05, 0xfc0d]);
}

#[test]
fn a_bootloader_that_does_not_answer_every_fragment_is_refused() {
    let mut m = ax201(0, 4);
    m.boot_params[20] = 1;
    let (r, m, said) = run(m, 0x0026);
    assert_eq!((r, m.fragments.len()), (Err(HciError::NotBluetooth), 0));
    assert_eq!(said, ["intel: bootloader does not answer every fragment"]);
}

#[test]
fn a_running_ax201_with_the_carried_build_is_left_running() {
    let mut m = ax201(0, 4);
    m.booted = true;
    let (r, m, _) = run(m, 0x0026);
    assert_eq!((r, m.ops), (Ok(()), vec![0xfc05, 0xfc52, 0x0c03]));
}
