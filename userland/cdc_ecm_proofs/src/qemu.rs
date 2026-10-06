// NONOS Operating System (AGPL-3.0-or-later)
//! QEMU's usb-net as a scripted device: its descriptors, and every other
//! control request accepted, or refused with `refuse` when it matches.

use nonos_usbnet::mock::{descriptors, Call, MockBus};
use nonos_usbnet::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE, MAC_STRING};

pub fn usb_net(refuse: Option<(u8, i32)>) -> MockBus {
    MockBus::new(Box::new(move |call| {
        let configs: [&[u8]; 2] = [&CONFIG_RNDIS, &CONFIG_ECM];
        if let Some(bytes) = descriptors(call, &DEVICE, &configs, &[(3, &MAC_STRING)]) {
            return Ok(bytes);
        }
        match (call, refuse) {
            (Call::Out(s, _), Some((request, e))) if s.request == request => Err(e),
            _ => Ok(vec![]),
        }
    }))
}
