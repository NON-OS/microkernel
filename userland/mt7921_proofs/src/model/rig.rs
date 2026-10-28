// NONOS Operating System (AGPL-3.0-or-later)

//! Putting the driver on the model: a chip, its register window, a claim,
//! the radio brought up by the driver's own `bringup::run_with` with the
//! MT7922-shaped synthetic package, and the shared station over it.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_wifi_core::softmac::SoftStation;

use super::package::{package, pick};
use super::{images, Chip, ModelBus, Shared};
use crate::bringup::run_with;
use crate::hw::Regs;
use crate::radio::Mt7921;
use crate::serve::env::Kernel;
use crate::setup::Mapped;
use crate::stage::Stage;

pub type Station = SoftStation<Mt7921, Kernel>;

pub fn chip() -> Shared {
    Rc::new(RefCell::new(Chip::new(0x7922)))
}

pub fn regs(c: &Shared) -> Regs {
    Regs::new(Box::new(ModelBus(c.clone())))
}

pub fn mapped() -> Mapped {
    Mapped { device_id: 1, epoch: 1, va: 0, len: 0x100000 }
}

/// Bring the driver up on `c` with the MT7922-shaped synthetic package.
pub fn bring_up(c: &Shared) -> (Option<Mt7921>, Stage) {
    package(&images::patch_7922(), &images::ram_7922(), true);
    run_with(regs(c), &mapped(), pick)
}

/// A radio up on a fresh chip, and the chip.
pub fn up() -> (Mt7921, Shared) {
    let c = chip();
    let (radio, stage) = bring_up(&c);
    assert_eq!(stage, Stage::Ready);
    (radio.expect("a ready radio"), c)
}

/// The shared station over a radio up on a fresh chip.
pub fn station() -> (Station, Shared) {
    let (radio, c) = up();
    (SoftStation::new(Some(radio), Kernel, 0), c)
}
