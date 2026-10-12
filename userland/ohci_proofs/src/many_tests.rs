// NONOS Operating System (AGPL-3.0-or-later)
//! Several OHCI controllers served as one host (server/many, shared with
//! driver.uhci0): ports numbered on by each controller's own count, and
//! a slot's requests reaching the controller it was addressed on.

use crate::model::{call_host, rig, Cx, Device};
use crate::server::many::front::Bind;
use crate::server::many::Front;
use crate::wire::ops::*;

#[test]
fn two_controllers_one_host() {
    let mut f: Front<Cx> = Front::new();
    let (a, _) = rig(vec![Some(Device::full(64)), None, None]);
    let (b, wb) = rig(vec![None, None, Some(Device::full(16))]);
    assert!(f.add(a) && f.add(b));
    let r = call_host(&mut f, OP_PORT_STATUS, &[]);
    let ids: Vec<u8> = r.data[4..].chunks(8).map(|e| e[0]).collect();
    assert_eq!((r.data[0], ids), (6, vec![1, 2, 3, 4, 5, 6]), "three ports each");
    let s = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    let addr = call_host(&mut f, OP_ADDRESS_DEVICE, &[s, 6]);
    assert_eq!((addr.status, addr.data[1], f.bound[s as usize - 1]), (0, 6, Bind::On(1)));
    assert_eq!(wb.borrow().devs[2].as_ref().unwrap().addr, s);
    let d = call_host(&mut f, OP_GET_DEVICE_DESCRIPTOR, &[s]);
    assert_eq!((d.status, d.data[7]), (0, 16), "the second controller's device");
}
