// NONOS Operating System (AGPL-3.0-or-later)
//! A modelled adapter plugged in: the scripted bus answering its
//! descriptors and the chip's vendor requests, and what the search read
//! from it before any driver was shown it.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_usbnet::found::{fetch, Found};
use nonos_usbnet::mock::{descriptors, MockBus};

use super::Chip;
use crate::adapter::{device, CONFIG_ECM, CONFIG_VENDOR};

pub struct Plugged {
    pub bus: MockBus,
    pub found: Found,
    pub chip: Rc<RefCell<Chip>>,
}

/// Realtek's own id, the vendor configuration first and ECM second.
pub fn plug(code: u16) -> Plugged {
    plug_with(code, (0x0bda, 0x8153), vec![CONFIG_VENDOR.to_vec(), CONFIG_ECM.to_vec()])
}

pub fn plug_with(code: u16, ids: (u16, u16), configs: Vec<Vec<u8>>) -> Plugged {
    let chip = Rc::new(RefCell::new(Chip::new(code)));
    chip.borrow_mut().start("autoload");
    let mut dev = device(ids.0, ids.1);
    dev[17] = configs.len() as u8;
    let answers = chip.clone();
    let bus = MockBus::new(Box::new(move |call| {
        let refs: Vec<&[u8]> = configs.iter().map(|c| c.as_slice()).collect();
        if let Some(d) = descriptors(call, &dev, &refs, &[]) {
            return Ok(d);
        }
        answers.borrow_mut().vendor(call).unwrap_or(Ok(vec![]))
    }));
    let found = fetch(&mut bus.clone(), 2).unwrap();
    bus.0.borrow_mut().calls.clear();
    Plugged { bus, found, chip }
}
