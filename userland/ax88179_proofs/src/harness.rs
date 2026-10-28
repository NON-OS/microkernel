// NONOS Operating System (AGPL-3.0-or-later)
//! The scripted chip behind MockBus, with the host's claim table and clock
//! as lists, and a chip bound.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_usbnet::found::{fetch, Found};
use nonos_usbnet::mock::{descriptors, drawn, MockBus};
use nonos_usbnet::Bind;

use crate::ax::bind;
use crate::ax::bring_up::Pause;
use crate::ax::claim::Claim;
use crate::ax::link::Ax;
use crate::chip::{answer, healthy, Chip, CONFIG, DEVICE};

thread_local! {
    /// Each wait, and how many calls the bus had seen before it.
    pub static PAUSED: RefCell<Vec<(u64, usize)>> = const { RefCell::new(Vec::new()) };
    pub static HELD: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

impl Pause for MockBus {
    fn pause(&mut self, ms: u64) {
        let at = self.0.borrow().calls.len();
        PAUSED.with(|p| p.borrow_mut().push((ms, at)));
    }
}

impl Claim for MockBus {
    fn claim(&mut self, _port: u8, interface: u8) -> Result<(), i32> {
        let held = HELD.with(|h| h.borrow().contains(&interface));
        if held {
            Err(-16)
        } else {
            Ok(())
        }
    }
}

pub type Shared = Rc<RefCell<Chip>>;

/// A chip of `device` offering `configs`, answering as `chip` says.
pub fn device_of(device: [u8; 18], configs: Vec<Vec<u8>>, chip: Chip) -> (MockBus, Found, Shared) {
    let chip = Rc::new(RefCell::new(chip));
    let c = chip.clone();
    let bus = MockBus::new(Box::new(move |call| {
        let raw: Vec<&[u8]> = configs.iter().map(|v| v.as_slice()).collect();
        let found = descriptors(call, &device, &raw, &[]);
        found.map_or_else(|| answer(&mut c.borrow_mut(), call), Ok)
    }));
    let found = fetch(&mut bus.clone(), 4).unwrap();
    bus.0.borrow_mut().calls.clear();
    PAUSED.with(|p| p.borrow_mut().clear());
    (bus, found, chip)
}

pub fn bound_with(chip: Chip) -> (MockBus, Ax<MockBus>, Shared) {
    let (bus, found, chip) = device_of(DEVICE, vec![CONFIG.to_vec()], chip);
    let Bind::Ours(ax) = bind(bus.clone(), &found, drawn()) else { panic!("not bound") };
    bus.0.borrow_mut().calls.clear();
    (bus, ax, chip)
}

pub fn bound() -> (MockBus, Ax<MockBus>, Shared) {
    bound_with(healthy())
}
