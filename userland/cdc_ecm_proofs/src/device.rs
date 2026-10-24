// NONOS Operating System (AGPL-3.0-or-later)
//! QEMU's usb-net with its ECM configuration or MAC string changed, and the
//! host's claim table as a list of interfaces another driver holds.

use std::cell::RefCell;

use nonos_usbnet::found::{fetch, Found};
use nonos_usbnet::mock::{descriptors, drawn, MockBus};
use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE, MAC_STRING};
use nonos_usbnet::Bind;

use crate::ecm::bind;
use crate::ecm::claim::Claim;
use crate::ecm::link::Ecm;

thread_local! {
    /// Interfaces another driver holds, and each hold this driver asked for.
    pub static HELD: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    pub static ASKED: RefCell<Vec<(u8, u8)>> = const { RefCell::new(Vec::new()) };
}

impl Claim for MockBus {
    fn claim(&mut self, port: u8, interface: u8) -> Result<(), i32> {
        ASKED.with(|a| a.borrow_mut().push((port, interface)));
        let held = HELD.with(|h| h.borrow().contains(&interface));
        if held {
            Err(-16)
        } else {
            Ok(())
        }
    }
}

/// The ECM configuration with `change` made to its bytes, and string 3 as
/// `mac`, which the driver never reads; every other request accepted.
pub fn device(change: impl Fn(&mut Vec<u8>), mac: &[u8]) -> (MockBus, Found) {
    let mut ecm = CONFIG_ECM.to_vec();
    change(&mut ecm);
    let mac = mac.to_vec();
    let found_ecm = ecm.clone();
    let bus = MockBus::new(Box::new(move |call| {
        let configs: [&[u8]; 2] = [&CONFIG_RNDIS, &found_ecm];
        Ok(descriptors(call, &DEVICE, &configs, &[(3, &mac)]).unwrap_or_default())
    }));
    let found = fetch(&mut bus.clone(), 2).unwrap();
    bus.0.borrow_mut().calls.clear();
    (bus, found)
}

pub fn bound(change: impl Fn(&mut Vec<u8>), mac: &[u8]) -> Result<(MockBus, Ecm<MockBus>), String> {
    let (bus, found) = device(change, mac);
    match bind(bus.clone(), &found, drawn()) {
        Bind::Ours(nic) => Ok((bus, nic)),
        Bind::NotOurs => Err("not ours".into()),
        Bind::Failed(what, e) => Err(format!("{what} {e}")),
    }
}

pub fn qemu() -> (MockBus, Ecm<MockBus>) {
    bound(|_| {}, &MAC_STRING).unwrap()
}
