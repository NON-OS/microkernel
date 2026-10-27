// NONOS Operating System (AGPL-3.0-or-later)
//! A host slot is the client's that enabled it: another is answered
//! E_PERM on every request naming it, park included; a slot given back is
//! the next enabler's; a gone client's slots are taken by the next that
//! asks, or given back by the sweep, but a child parked below a hub.

use crate::model::hub::HubModel;
use crate::model::{call_as, rig, Cx, Device, CLIENT};
use crate::server::many::front::Bind;
use crate::server::many::Front;
use crate::virt_fixture::Tree;
use crate::wire::ops::*;
use crate::wire::{E_PERM, HDR_LEN, REPLY_MAX, STATUS_LEN};
use nonos_usb_virt::OP_PARK;

const OTHER: u32 = 8;

fn host() -> (Front<Cx>, u8) {
    let mut f = Front::new();
    assert!(f.add(rig([Some(Device::full(64)), Some(Device::full(64))]).0));
    let s = call_as(&mut f, CLIENT, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call_as(&mut f, CLIENT, OP_ADDRESS_DEVICE, &[s, 1]).status, 0);
    (f, s)
}

#[test]
fn another_client_is_refused_every_request_on_a_slot_it_did_not_enable() {
    let (mut f, s) = host();
    for (op, body) in [(OP_GET_DEVICE_DESCRIPTOR, vec![s]), (OP_DISABLE_SLOT, vec![s])] {
        assert_eq!(call_as(&mut f, OTHER, op, &body).status, E_PERM, "op {op:#x}");
    }
    let mine = call_as(&mut f, OTHER, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call_as(&mut f, CLIENT, OP_ADDRESS_DEVICE, &[mine, 2]).status, E_PERM);
    let park = [mine, s, 1, 2, 1, 0, 0, 0, 0, 0, 64, 0];
    assert_eq!(call_as(&mut f, OTHER, OP_PARK, &park).status, E_PERM, "the hub is not its");
    assert_eq!(f.bound[s as usize - 1], Bind::On(0));
    assert_eq!(call_as(&mut f, CLIENT, OP_GET_DEVICE_DESCRIPTOR, &[s]).status, 0);
    assert_eq!(call_as(&mut f, CLIENT, OP_DISABLE_SLOT, &[s]).status, 0);
    let again = call_as(&mut f, OTHER, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(again, s, "given back, it is the next enabler's");
    assert_eq!(call_as(&mut f, CLIENT, OP_ADDRESS_DEVICE, &[s, 1]).status, E_PERM);
    assert_eq!(call_as(&mut f, OTHER, OP_ADDRESS_DEVICE, &[s, 1]).status, 0);
}

#[test]
fn a_gone_clients_slot_is_the_next_askers_or_the_sweeps() {
    let (mut f, s) = host();
    f.alive = |pid| pid != CLIENT;
    assert_eq!(call_as(&mut f, OTHER, OP_GET_DEVICE_DESCRIPTOR, &[s]).status, 0);
    assert_eq!(f.owner[s as usize - 1], OTHER, "taken over");
    let (mut f, s) = host();
    f.alive = |pid| pid != CLIENT;
    f.sweep(&mut [0u8; HDR_LEN + STATUS_LEN + REPLY_MAX]);
    assert_eq!(f.bound[s as usize - 1], Bind::Free);
    assert!(f.hcs[0].as_mut().unwrap().table.dev(s).is_none(), "disabled on its controller");
}

#[test]
fn the_sweep_keeps_the_children_a_gone_hub_driver_parked() {
    let mut t = Tree::new(HubModel::new(false, 4).plug(3, Device::full(64), 1));
    t.f.alive = |pid| pid != CLIENT;
    t.f.sweep(&mut [0u8; HDR_LEN + STATUS_LEN + REPLY_MAX]);
    assert_eq!(t.f.bound[0], Bind::Free, "the hub's slot given back");
    let s = call_as(&mut t.f, OTHER, OP_ENABLE_SLOT, &[]).data[0];
    let a = call_as(&mut t.f, OTHER, OP_ADDRESS_DEVICE, &[s, 0x80]);
    assert_eq!((a.status, a.data[0]), (0, s), "the child still there to claim");
}
