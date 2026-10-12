// NONOS Operating System (AGPL-3.0-or-later)
//! The class side of the hub lane, shared by the host proof crates: a
//! hub's configuration, a hub device carrying it, and classes that bring
//! a hub up with the hub class's own `attach` and bind anything else.

use std::cell::RefMut;

use nonos_usb_hub::{attach, Classes, Host, Hub, Place, Setup};

use crate::hub_lane::Lane;
use crate::hub_wire::Wire;
use crate::model::hub::HubModel;
use crate::model::{Device, Shared};

/// One interface of class 09h and its status endpoint 81h, interval 12.
pub const HUB_CONFIG: [u8; 25] =
    [9, 2, 25, 0, 1, 1, 0, 0xE0, 0, 9, 4, 0, 0, 1, 9, 0, 0, 0, 7, 5, 0x81, 3, 1, 0, 12];

#[derive(Default)]
pub struct Seen {
    pub bound: Vec<(u8, Place)>,
}

impl<'a, W: Wire> Classes<Lane<'a, W>> for Seen {
    fn classify(
        &mut self,
        h: &mut Lane<'a, W>,
        slot: u8,
        place: Place,
        hubs: &mut Vec<Hub>,
    ) -> bool {
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

/// The device on the root hub's `port`.
pub fn child(w: &Shared, port: usize) -> RefMut<'_, Device> {
    RefMut::map(w.borrow_mut(), |w| {
        let hub = w.devs[0].as_mut().unwrap().hub.as_mut().unwrap();
        hub.ports[port - 1].dev.as_mut().unwrap()
    })
}
