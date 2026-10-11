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

//! Every configuration word the driver writes, from every value the
//! register could hold, held to the kernel broker's own rule: its
//! quirk_bits module is mounted here unmodified. A word the broker would
//! refuse fails the chip reset on the machine; these find it first.

use crate::chip::pcicfg::{device_control, device_status, host_control, pci_state};
use crate::constants::pcicfg::*;
use crate::quirk_bits::{clear_only, cleared_only, only, writable, Ident};

const CAP: u16 = 0xAC;
const TG3: Ident = Ident { vendor: 0x14E4, class: 0x02, subclass: 0x00, pcie_cap: Some(CAP) };

fn every(f: fn(u16) -> u16, rule: impl Fn(u16, u16) -> bool) {
    for current in 0..=u16::MAX {
        assert!(rule(f(current), current), "{current:#06x} -> {:#06x}", f(current));
    }
}

#[test]
fn host_control_and_pci_state_stay_inside_their_bits() {
    let hc = writable(&TG3, TG3PCI_MISC_HOST_CTRL).unwrap();
    every(host_control, |new, cur| only(hc, new, cur));
    let ps = writable(&TG3, TG3PCI_PCISTATE).unwrap();
    every(pci_state, |new, cur| only(ps, new, cur));
}

#[test]
fn device_status_clears_only_its_error_bits() {
    let sta = writable(&TG3, CAP as u32 + PCI_EXP_DEVSTA).unwrap();
    every(device_status, |new, cur| only(sta, new, cur));
}

#[test]
fn device_control_only_ever_clears() {
    let ctl = clear_only(&TG3, CAP as u32 + PCI_EXP_DEVCTL).unwrap();
    every(device_control, |new, cur| cleared_only(ctl, new, cur));
    assert_eq!(device_control(0xFFFF), !0x0810);
}

#[test]
fn the_words_are_linuxs() {
    // tp->misc_host_ctrl below the revision: MASK_PCI_INT, WORD_SWAP,
    // PCISTATE_RW and INDIR_ACCESS.
    assert_eq!(MISC_HOST_CTRL, 0x9A);
    assert_eq!(host_control(0x0000), 0x009A);
    assert_eq!(host_control(0xFFFF), 0xFC9A);
    assert_eq!(pci_state(0x0000), 0x0060);
    assert_eq!(device_status(0x0000), 0x000F);
}

#[test]
fn a_function_the_rule_does_not_name_gets_nothing() {
    let other = Ident { vendor: 0x8086, ..TG3 };
    assert_eq!(writable(&other, TG3PCI_MISC_HOST_CTRL), None);
    assert_eq!(clear_only(&other, CAP as u32 + PCI_EXP_DEVCTL), None);
    let wifi = Ident { subclass: 0x80, ..TG3 };
    assert_eq!(writable(&wifi, TG3PCI_PCISTATE), None);
}
