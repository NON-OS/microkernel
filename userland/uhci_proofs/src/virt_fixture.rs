// NONOS Operating System (AGPL-3.0-or-later)
//! A full-speed hub on root port 1 of a one-controller host front, walked
//! as driver.usb_hub0 walks it (nonos_usb_hub's Parker): its children
//! parked with the host as virtual ports for the class drivers to claim.

use nonos_usb_hub::{as_hub, Hub, Parker, Place};

use crate::hub_fixture::settle;
use crate::hub_lane::Lane;
use crate::hub_seen::hub_dev;
use crate::model::hub::HubModel;
use crate::model::{call_host, rig, Answer, Cx, Shared};
use crate::server::many::Front;
use crate::wire::ops::*;

/// A PORT_STATUS entry as (port, owner, kind, PORTSC).
pub type Entry = (u8, u8, u8, u32);

pub struct Tree {
    pub f: Front<Cx>,
    pub w: Shared,
    pub hubs: Vec<Hub>,
}

impl Tree {
    pub fn new(model: HubModel) -> Self {
        let (cx, w) = rig([Some(hub_dev(model)), None]);
        let mut f = Front::new();
        assert!(f.add(cx));
        let slot = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
        assert_eq!(call_host(&mut f, OP_ADDRESS_DEVICE, &[slot, 1]).status, 0);
        let mut lane = Lane { cx: &mut f, lines: vec![] };
        let hub = as_hub(&mut lane, slot, Place::root(1, 1)).expect("a hub on port 1");
        let mut t = Tree { f, w, hubs: vec![hub] };
        t.walk();
        t
    }

    /// One pass of driver.usb_hub0 over the hubs it holds.
    pub fn walk(&mut self) {
        let mut lane = Lane { cx: &mut self.f, lines: vec![] };
        settle(&mut lane, &mut self.hubs, &mut Parker);
    }

    pub fn call(&mut self, op: u16, body: &[u8]) -> Answer {
        call_host(&mut self.f, op, body)
    }

    /// A class driver's claim on `vport`: its own slot, and the answer.
    pub fn claim(&mut self, vport: u8) -> (u8, Answer) {
        let s = self.call(OP_ENABLE_SLOT, &[]).data[0];
        (s, self.call(OP_ADDRESS_DEVICE, &[s, vport]))
    }

    /// PORT_STATUS's entries and its raw extensions.
    pub fn ports(&mut self) -> (Vec<Entry>, Vec<[u8; 8]>) {
        let r = self.call(OP_PORT_STATUS, &[]);
        let (n, v) = (r.data[0] as usize, r.data[1] as usize);
        let e = r.data[4..4 + 8 * n].chunks(8);
        let sc = |e: &[u8]| u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
        let entries = e.map(|e| (e[0], e[1], e[2], sc(e))).collect();
        let x = r.data[4 + 8 * n..4 + 8 * (n + v)].chunks(8);
        (entries, x.map(|c| c.try_into().unwrap()).collect())
    }

    /// The child on the hub's `port` leaves.
    pub fn unplug(&mut self, port: usize) {
        let mut w = self.w.borrow_mut();
        let hub = w.devs[0].as_mut().unwrap().hub.as_mut().unwrap();
        let p = &mut hub.ports[port - 1];
        (p.dev, p.enabled, p.c_connection) = (None, false, true);
    }
}
