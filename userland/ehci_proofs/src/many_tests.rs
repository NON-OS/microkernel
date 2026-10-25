// NONOS Operating System (AGPL-3.0-or-later)
//! An ICH's two EHCI controllers served as one host (server/many, shared
//! with the companions' drivers): ports numbered on, and a slot's requests
//! reaching the controller it was addressed on.

use crate::model::{call_host, high, rig, Cx};
use crate::server::many::front::Bind;
use crate::server::many::Front;
use crate::wire::ops::*;

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
