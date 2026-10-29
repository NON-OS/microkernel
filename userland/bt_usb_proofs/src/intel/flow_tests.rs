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

use super::loader::VID;
use super::rig::{bootloader, run, tlv, BZRU, TYP};
use super::rig_ops::ops_after;
use nonos_bt_hci::HciError;

#[test]
fn an_ax210_is_loaded_booted_given_its_ddc_masked_and_reset() {
    let (r, m, said) = run(bootloader(TYP, 1), 0x0032);
    assert_eq!(r, Ok(()));
    assert_eq!((m.boot_addr, m.booted, m.to_bootloader), (Some(0x0010_0800), true, 0));
    assert_eq!(ops_after(&m, 0xfc09), [0xfc01, 0xfc05, 0xfc8b, 0xfc8b, 0xfc52, 0x0c03]);
    assert_eq!(said, ["intel: sending ibt-0041-0041", "intel: booted, DDC from ibt-0041-0041"]);
}

#[test]
fn events_on_bulk_in_and_a_result_before_the_last_complete_are_taken() {
    let mut m = bootloader(BZRU, 1);
    (m.bulk_events, m.result_first) = (true, true);
    m.tlv[1] = tlv(BZRU, 0x03, [0x91, 0x24, 0x1a], 1);
    let (r, m, _) = run(m, 0x0036);
    assert_eq!(r, Ok(()));
    assert_eq!((m.booted, m.ddc.len()), (true, 1));
}

#[test]
fn a_controller_running_the_carried_build_is_not_loaded() {
    let mut m = bootloader(TYP, 1);
    m.booted = true;
    let (r, m, said) = run(m, 0x0032);
    assert_eq!(r, Ok(()));
    assert_eq!(m.ops, [0xfc05, 0xfc52, 0x0c03]);
    assert!(m.fragments.is_empty() && m.ddc.is_empty() && m.boot_addr.is_none());
    assert_eq!(said, ["intel: firmware already running: ibt-0041-0041"]);
}

#[test]
fn a_controller_running_another_build_is_sent_back_to_its_bootloader() {
    let mut m = bootloader(TYP, 1);
    (m.booted, m.tlv[1]) = (true, tlv(TYP, 0x03, [0x18, 0x1d, 0x19], 1));
    let (r, m, _) = run(m, 0x0032);
    assert_eq!(r, Err(HciError::NotBluetooth));
    assert_eq!((m.ops.as_slice(), m.to_bootloader), (&[0xfc05, 0xfc01][..], 1));
}

#[test]
fn a_product_linux_does_not_mark_combined_is_left_to_bring_up() {
    assert_eq!(VID, 0x8087, "the vendor the capsule's hook matches");
    let (r, m, said) = run(bootloader(TYP, 1), 0x0a5a);
    assert_eq!(r, Ok(()));
    assert!(m.ops.is_empty() && said.is_empty());
}
