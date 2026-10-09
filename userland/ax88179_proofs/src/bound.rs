// NONOS Operating System (AGPL-3.0-or-later)
//! A device as the search hands it to the driver, and the scripted chip
//! bound through it.

use nonos_usbnet::desc::DeviceInfo;
use nonos_usbnet::mock::MockBus;
use nonos_usbnet::{Bind, Found};

use crate::ax::bind;
use crate::ax::link::Ax88179;
use crate::chip::Chip;
use crate::layout::CONFIG;

pub fn found(vendor: u16, product: u16, class: u8, configs: &[&[u8]]) -> Found {
    let info = DeviceInfo { vendor, product, class, configs: configs.len() as u8 };
    Found { port: 1, info, configs: configs.iter().map(|c| c.to_vec()).collect() }
}

/// The chip bound as an AX88179 (0b95:1790) in its vendor configuration.
pub fn bound(chip: &Chip) -> (MockBus, Ax88179<MockBus>) {
    let bus = chip.bus();
    let Bind::Ours(nic) = bind(bus.clone(), &found(0x0b95, 0x1790, 0xff, &[&CONFIG])) else {
        panic!("not bound")
    };
    (bus, nic)
}
