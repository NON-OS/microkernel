// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The HP's controller brought up as the capsule does it: the RTL8821C
//! ROM named, its ROM version read, the patch sent in 139 numbered
//! fragments, then HCI Reset and the standard bring-up. A controller of
//! another vendor is sent no vendor command at all.

use super::firmware::{CONFIG, FW};
use super::rom::{downloaded, image, patched};
use crate::air::Air;
use crate::fixture::{air, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::smp_sim::Peer;
use nonos_bt_rtl::{prepare, Outcome};
use nonos_bt_smp::method::IoCap;

fn opcodes(air: &Air) -> Vec<u16> {
    air.commands.iter().map(|c| u16::from_le_bytes([c[0], c[1]])).collect()
}

#[test]
fn the_patch_goes_first_then_reset_then_bring_up() {
    let (mut host, _, _, out) = patched(Peer::new(IoCap::KeyboardOnly, true));
    assert_eq!(out, Outcome::Patched { rom: 1, fw_version: 0x75b8_f098, fragments: 139 });
    let ops = opcodes(air(&mut host));
    assert_eq!(ops[..2], [0x1001, 0xfc6d]);
    assert!(ops[2..141].iter().all(|&o| o == 0xfc20), "139 fragments, nothing between");
    assert_eq!(ops[141], 0x0c03, "HCI Reset straight after the last fragment");
    assert!(ops[142..].iter().all(|&o| o >> 10 != 0x3f), "a vendor command after the patch");
}

#[test]
fn the_fragments_carry_the_patch_and_config_in_order() {
    let (mut host, _, _, _) = patched(Peer::new(IoCap::KeyboardOnly, true));
    let (img, idx) = downloaded(air(&mut host));
    assert_eq!(img, image());
    let want: Vec<u8> = (0..=0x7f).chain(1..=10).chain([0x8b]).collect();
    assert_eq!(idx, want);
}

/// The capsule's own controller run does what `patched` does: the patch,
/// then bring-up, and a controller whose patch failed is not brought up.
#[test]
fn the_capsule_loads_the_patch_before_bring_up() {
    let run = include_str!("../../../capsule_driver_bt_usb/src/run/controller.rs");
    let (patch, up) = (run.find("match patch(&mut host, b.ids) {"), run.find("host.bring_up()"));
    assert!(patch.zip(up).is_some_and(|(p, u)| p < u), "bring-up before the patch");
    assert!(run.contains("Ok(()) => run_on(&mut host, irk, env),\n        Err(e) => e,"));
}

#[test]
fn another_vendors_controller_gets_no_vendor_command() {
    let (mut host, _, _) = with(Peer::new(IoCap::KeyboardOnly, true), keyboard_mouse(), &HID_ADV);
    let before = air(&mut host).commands.len();
    assert_eq!(prepare(&mut host, (0x8087, 0x0026), FW, CONFIG), Ok(Outcome::NotRealtek));
    assert_eq!(air(&mut host).commands.len(), before);
    assert!(opcodes(air(&mut host)).iter().all(|&o| o >> 10 != 0x3f));
}
