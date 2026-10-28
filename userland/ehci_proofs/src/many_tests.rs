// NONOS Operating System (AGPL-3.0-or-later)
//! An ICH's two EHCI controllers served as one host (server/many, shared
//! with the companions' drivers): ports numbered on, and a slot's requests
//! reaching the controller it was addressed on.

use crate::model::{call_as, call_host, high, rig, Cx, CLIENT};
use crate::server::many::front::Bind;
use crate::server::many::Front;
use crate::wire::ops::*;
use crate::wire::E_PERM;

#[test]
fn two_controllers_one_host() {
    let mut f: Front<Cx> = Front::new();
    let (a, _) = rig(vec![high(), None]);
    let (b, wb) = rig(vec![None, high()]);
    let per = a.table.ports;
    assert!(f.add(a) && f.add(b));
    let r = call_host(&mut f, OP_PORT_STATUS, &[]);
    assert_eq!(r.data[0], 2 * per);
    let s = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    let port = per + 2;
    let addr = call_host(&mut f, OP_ADDRESS_DEVICE, &[s, port]);
    assert_eq!((addr.status, addr.data[1], f.bound[s as usize - 1]), (0, port, Bind::On(1)));
    assert_eq!(wb.borrow().devs[1].as_ref().unwrap().addr, s);
    assert_eq!(call_host(&mut f, OP_GET_DEVICE_DESCRIPTOR, &[s]).status, 0);
}

#[test]
fn a_slot_serves_only_the_client_that_enabled_it() {
    let mut f: Front<Cx> = Front::new();
    assert!(f.add(rig(vec![high(), None]).0));
    assert_eq!(call_host(&mut f, OP_PORT_STATUS, &[]).status, 0, "each port decided");
    let s = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call_host(&mut f, OP_ADDRESS_DEVICE, &[s, 1]).status, 0);
    for op in [OP_GET_DEVICE_DESCRIPTOR, OP_DISABLE_SLOT] {
        assert_eq!(call_as(&mut f, CLIENT + 1, op, &[s]).status, E_PERM);
    }
    assert_eq!(call_host(&mut f, OP_GET_DEVICE_DESCRIPTOR, &[s]).status, 0);
}
