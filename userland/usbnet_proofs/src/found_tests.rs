// NONOS Operating System (AGPL-3.0-or-later)
//! Reading a device's descriptors over endpoint 0.

use crate::found::fetch;
use crate::mock::{descriptors, MockBus};
use crate::qemu_usb_net::{CONFIG_ECM, CONFIG_RNDIS, DEVICE, MAC_STRING};

fn qemu() -> MockBus {
    MockBus::new(Box::new(|call| {
        let configs: [&[u8]; 2] = [&CONFIG_RNDIS, &CONFIG_ECM];
        descriptors(call, &DEVICE, &configs, &[(3, &MAC_STRING)]).ok_or(-32)
    }))
}

#[test]
fn every_configuration_is_read_whole() {
    let mut bus = qemu();
    let found = fetch(&mut bus, 4).unwrap();
    assert_eq!(found.port, 4);
    assert_eq!(found.configs.len(), 2);
    assert_eq!(found.configs[0], CONFIG_RNDIS.to_vec());
    assert_eq!(found.configs[1], CONFIG_ECM.to_vec());
}

#[test]
fn a_device_descriptor_that_is_not_one_fails() {
    let mut bus = MockBus::new(Box::new(|_| Ok(vec![0x12, 0x02, 0, 0])));
    assert_eq!(fetch(&mut bus, 1).err(), Some(-5));
}

#[test]
fn a_stall_on_a_configuration_fails_with_its_errno() {
    let mut bus =
        MockBus::new(Box::new(|call| descriptors(call, &DEVICE, &[&CONFIG_RNDIS], &[]).ok_or(-32)));
    assert_eq!(fetch(&mut bus, 1).err(), Some(-32));
}
