// NONOS Operating System (AGPL-3.0-or-later)
//! Children below a hub as the host lists and lends them: virtual ports
//! in PORT_STATUS, claimed as root ports are, first claim winning, and a
//! detach reaching the claimant as a root-port detach does.

use crate::hub_seen::child;
use crate::model::hub::HubModel;
use crate::model::Device;
use crate::virt_fixture::Tree;
use crate::wire::ops::*;
use crate::wire::{E_BUSY, E_INVAL};

fn tree() -> Tree {
    Tree::new(HubModel::new(false, 4).plug(2, Device::low(), 2).plug(3, Device::full(64), 1))
}

#[test]
fn parked_children_are_listed_beside_the_root_ports() {
    let mut t = tree();
    let (entries, exts) = t.ports();
    let ids: Vec<(u8, u8, u8)> = entries.iter().map(|e| (e.0, e.1, e.2)).collect();
    assert_eq!(ids, vec![(1, 2, 0), (2, 0, 0), (0x80, 0, 1), (0x81, 0, 1)], "hub claimed");
    assert_eq!(entries[2].3 & 0x3, 0x3, "connected and enabled");
    assert_eq!((entries[2].3 >> 10) & 0xF, 2, "low speed");
    assert_ne!(entries[3].3 & (1 << 17), 0, "a new connection, told once");
    assert_eq!(exts, vec![[0x80, 1, 0, 0, 2, 0, 0, 0], [0x81, 1, 0, 0, 3, 0, 0, 0]]);
    assert_eq!(t.ports().0[3].3 & (1 << 17), 0);
}

#[test]
fn a_claim_is_answered_as_an_address_and_its_slot_reaches_the_child() {
    let mut t = tree();
    let (s, a) = t.claim(0x81);
    assert_eq!((a.status, &a.data[..6]), (0, &[s, 0x81, 1, 0, 64, 0][..]));
    assert_eq!(t.claim(0x81).1.status, E_BUSY, "the first claim wins");
    assert_eq!(t.call(OP_CONFIGURE_BULK, &[s, 0x82, 0x03, 0, 64, 0, 64, 0]).status, 0);
    let mut body = vec![s, 0, 10, 0];
    body.extend([7u8; 10]);
    assert_eq!(t.call(OP_BULK_OUT, &body).status, 0);
    assert_eq!(child(&t.w, 3).bulk_out, vec![7u8; 10]);
    assert_eq!(t.ports().0[3].1, 2, "claimed, and an endpoint configured");
}

#[test]
fn a_child_that_leaves_reaches_its_claimant_then_goes() {
    let mut t = tree();
    let (s, _) = t.claim(0x80);
    t.unplug(2);
    t.walk();
    let gone = t.ports().0[2];
    assert_eq!((gone.0, gone.3 & 1, gone.3 >> 17 & 1), (0x80, 0, 1), "read as a detach");
    assert_eq!(t.call(OP_DISABLE_SLOT, &[s]).status, 0);
    assert!(t.ports().0.iter().all(|e| e.0 != 0x80), "the port is gone with both slots");
    assert!(t.f.hcs[0].as_mut().unwrap().table.dev(2).is_none(), "the parked slot too");
}

#[test]
fn a_claimant_letting_go_leaves_the_child_parked_for_the_next() {
    let mut t = tree();
    let (s, _) = t.claim(0x81);
    assert_eq!(t.call(OP_DISABLE_SLOT, &[s]).status, 0);
    assert_eq!(t.ports().0[3].1, 0, "free again");
    assert_eq!(t.claim(0x81).1.status, 0);
    let park = [9, 1, 2, 1, 2, 0, 0, 0, 0, 0, 8, 0];
    assert_eq!(t.call(nonos_usb_virt::OP_PARK, &park).status, E_INVAL, "slot 9 is no one's");
}
