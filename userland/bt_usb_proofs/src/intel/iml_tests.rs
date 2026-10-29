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

//! The BE201 on Meteor Lake and Arrow Lake CNVi: a BlazarI B0 with an FM
//! radio, which takes its intermediate loader, then its DSBR, then the
//! firmware the loader names by its id ("usb").

use super::loader::carried::carried;
use super::model::Intel;
use super::name_tests::BZRI;
use super::rig::{run, tlv};
use super::rig_ops::named;
use nonos_bt_hci::HciError;

const LOADER_BUILD: [u8; 3] = [0x3e, 0x21, 0x1a];

/// In its bootloader; the loader reports the id `id`, the firmware none.
fn be201(id: &[u8]) -> Intel {
    let (iml, usb) = (carried("ibt-0190-0291-iml").unwrap(), carried("ibt-0190-0291-usb").unwrap());
    let loading = named(tlv(BZRI, 0x02, [0; 3], 1), id);
    let tlv = [tlv(BZRI, 0x01, [0; 3], 1), tlv(BZRI, 0x03, LOADER_BUILD, 1)];
    let (iml, expect) = (Some((loading, usb.sfi.len() - 964)), iml.sfi.len() - 964);
    Intel { tlv, iml, boots: true, expect, ..Intel::default() }
}

#[test]
fn a_be201_takes_its_loader_its_dsbr_then_the_firmware_the_loader_names() {
    let (r, m, said) = run(be201(b"usb"), 0x0037);
    assert_eq!(r, Ok(()));
    let mut ops = m.ops.clone();
    ops.dedup();
    let want = [0xfc05, 0xfc09, 0xfc01, 0xfc05, 0xfc0a, 0xfc09, 0xfc01, 0xfc05, 0xfc8b];
    assert_eq!(ops, [&want[..], &[0xfc52, 0x0c03]].concat());
    assert_eq!(m.boot_addrs, [0x3009_8800, 0x1000_0800]);
    let (iml, usb) = (carried("ibt-0190-0291-iml").unwrap(), carried("ibt-0190-0291-usb").unwrap());
    let data: Vec<u8> =
        m.fragments.iter().filter(|f| f[0] == 0x01).flat_map(|f| f[1..].to_vec()).collect();
    assert_eq!(data, [&iml.sfi[964..], &usb.sfi[964..]].concat());
    assert_eq!(m.ddc.concat(), usb.ddc, "DDC named from the loader's version, not the firmware's");
    let sent = ["intel: sending ibt-0190-0291-iml", "intel: sending ibt-0190-0291-usb"];
    assert_eq!(said, [&sent[..], &["intel: booted, DDC from ibt-0190-0291-usb"]].concat());
}

#[test]
fn a_loader_naming_no_carried_firmware_is_refused_after_it_boots() {
    let (r, m, said) = run(be201(b""), 0x0037);
    assert_eq!((r, m.boot_addrs.len(), m.ddc.len()), (Err(HciError::NotBluetooth), 1, 0));
    assert_eq!(said.last().unwrap(), "intel: firmware not carried: ibt-0190-0291.sfi");
}

#[test]
fn a_running_be201_is_matched_against_its_loader_build() {
    let mut m = be201(b"usb");
    m.booted = true;
    let (r, m, said) = run(m, 0x0037);
    assert_eq!((r, m.ops), (Ok(()), vec![0xfc05, 0xfc52, 0x0c03]));
    assert_eq!(said, ["intel: firmware already running: ibt-0190-0291-iml"]);
    let mut m = be201(b"usb");
    (m.booted, m.tlv[1]) = (true, tlv(BZRI, 0x03, [0x3e, 0x21, 0x19], 1));
    assert_eq!((run(m, 0x0037).1).to_bootloader, 1);
}
