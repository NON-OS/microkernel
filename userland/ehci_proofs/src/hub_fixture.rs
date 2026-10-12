// NONOS Operating System (AGPL-3.0-or-later)
//! A high-speed hub on root port 1, addressed and brought up by the hub
//! class's own `attach`, with what the tests plug into its ports; and the
//! class side that binds whatever is not a hub.

use std::cell::RefMut;

use nonos_usb_hub::{attach, scan_hubs, Classes, Host, Hub, Place, Setup};

use crate::hub_lane::Lane;
use crate::model::hub::HubModel;
use crate::model::{addressed, rig, Cx, Device, Shared, Speed};

/// One interface of class 09h and its status endpoint 81h, interval 12.
pub const HUB_CONFIG: [u8; 25] =
    [9, 2, 25, 0, 1, 1, 0, 0xE0, 0, 9, 4, 0, 0, 1, 9, 0, 0, 0, 7, 5, 0x81, 3, 1, 0, 12];

#[derive(Default)]
pub struct Seen {
    pub bound: Vec<(u8, Place)>,
}

impl<'a> Classes<Lane<'a>> for Seen {
    fn classify(&mut self, h: &mut Lane<'a>, slot: u8, place: Place, hubs: &mut Vec<Hub>) -> bool {
        let mut raw = [0u8; 64];
        let get = Setup { request_type: 0x80, request: 6, value: 0x0200, index: 0, length: 64 };
        let Ok(n) = h.control(slot, get, &mut raw) else { return false };
        if n > 14 && raw[14] == 9 {
            hubs.extend(attach(h, slot, place, &raw[..n]).ok());
            return false;
        }
        self.bound.push((slot, place));
        true
    }
    fn forget(&mut self, _slot: u8) {}
}

pub fn hub_dev(model: HubModel) -> Device {
    let mut d = Device::full(64);
    (d.hub, d.config) = (Some(Box::new(model)), HUB_CONFIG.to_vec());
    d
}

/// The hub with `model`'s ports on root port 1, walked once.
pub fn tree(model: HubModel) -> (Cx, Shared, Vec<Hub>, Seen) {
    let (mut cx, w) = rig(vec![Some((hub_dev(model), Speed::High)), None]);
    let slot = addressed(&mut cx, 1);
    let mut lane = Lane { cx: &mut cx, lines: vec![] };
    let mut hubs = vec![attach(&mut lane, slot, Place::root(1, 3), &HUB_CONFIG).expect("hub up")];
    let mut seen = Seen::default();
    scan_hubs(&mut lane, &mut hubs, &mut seen);
    (cx, w, hubs, seen)
}

/// The device on the root hub's `port`.
pub fn child(w: &Shared, port: usize) -> RefMut<'_, Device> {
    RefMut::map(w.borrow_mut(), |w| {
        let hub = w.devs[0].as_mut().unwrap().hub.as_mut().unwrap();
        hub.ports[port - 1].dev.as_mut().unwrap()
    })
}
