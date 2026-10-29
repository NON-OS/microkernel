// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The air's controller as the HP's RTL8821C ROM answers before its
//! patch: Read Local Version names the ROM (LMP subversion 0x8821, HCI
//! revision 0x000c, version 8, Realtek), Read ROM Version gives 1, and each
//! download fragment is taken with its index (btrtl.c). The capsule's own
//! sequence then runs on it: the patch, bring-up, links.

use std::cell::Cell;
use std::rc::Rc;

use super::board_env::BoardEnv;
use super::firmware::{CONFIG, FW};
use crate::air::Air;
use crate::fixture::{Stack, HID_ADV};
use crate::hci_sim::events::complete;
use crate::hci_sim::stack::IFACE;
use crate::hogp_sim::{keyboard_mouse, Peripheral};
use crate::smp_sim::Peer;
use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::Host;
use nonos_bt_rtl::{prepare, Outcome};
use nonos_bt_wire::addr::BdAddr;

pub const HP: (u16, u16) = (0x0bda, 0xb00a);
const ROM_VERSION: [u8; 8] = [0x08, 0x0c, 0x00, 0x08, 0x5d, 0x00, 0x21, 0x88];

/// The image ROM 1 is sent, from the file's header as a hex dump reads it:
/// the patch at 0x4cc0 of 0x8864 octets, its last four the version
/// 0x75b8f098, then the config.
pub fn image() -> Vec<u8> {
    [&FW[0x4cc0..0x4cc0 + 0x8864 - 4], &0x75b8_f098u32.to_le_bytes()[..], CONFIG].concat()
}

/// A peer on the air behind an unpatched RTL8821C, patched and brought up
/// as the capsule does it: what the patch did, the host and its address.
pub fn patched(smp: Peer) -> (Stack, BdAddr, BoardEnv, Outcome) {
    let shown = Rc::new(Cell::new(None));
    let mut air = Air::new(&HID_ADV, smp, Peripheral::new(keyboard_mouse()), shown.clone());
    air.cfg.script = vec![
        (0x1001, vec![complete(1, 0x1001, 0, &ROM_VERSION)]),
        (0xfc6d, vec![complete(1, 0xfc6d, 0, &[1])]),
        (0xfc20, vec![complete(1, 0xfc20, 0, &[0])]),
    ];
    let mut host = Host::new(UsbHci::new(air, &IFACE, 27));
    let out = prepare(&mut host, HP, FW, CONFIG).expect("the patch loads");
    let c = host.bring_up().expect("bring-up after the patch");
    host.transport().set_acl_max(c.acl_len);
    let mut env = BoardEnv::new(shown);
    env.board.controller = true;
    (host, c.addr, env, out)
}

/// What the fragments carried, in order, and their index octets.
pub fn downloaded(air: &Air) -> (Vec<u8>, Vec<u8>) {
    let frags = air.commands.iter().filter(|c| c[..2] == [0x20, 0xfc]);
    frags.fold((Vec::new(), Vec::new()), |(mut img, mut idx), c| {
        idx.push(c[3]);
        img.extend_from_slice(&c[4..]);
        (img, idx)
    })
}
