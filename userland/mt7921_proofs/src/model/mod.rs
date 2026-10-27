// NONOS Operating System (AGPL-3.0-or-later)

//! The modelled chip. It sits behind the driver's `Bus`, so every register
//! access the driver makes reaches it; ring descriptors and buffers are the
//! driver's own DMA grants, which the model reads and writes by device
//! address, as the WFDMA would. Its behaviour is what Linux v6.12's
//! driver relies on: ownership handshakes, the L1 remap window, the
//! subsystem reset, the rings' indices, the MCU's command protocol and
//! firmware download, and an access point on the air.

pub mod air;
pub mod chip;
pub mod data;
pub mod faults;
pub mod images;
mod l1;
pub mod mcu;
mod mcu_cmds;
pub mod mcu_legacy;
mod mcu_uni;
pub mod mem;
pub mod net;
pub mod package;
pub mod reply;
pub mod rig;
pub mod rings;
mod rx;
pub mod wire;

use std::cell::RefCell;
use std::rc::Rc;

use crate::hw::bus::Bus;

pub use chip::Chip;
pub use faults::Faults;

pub type Shared = Rc<RefCell<Chip>>;

/// The model as the driver's register window.
pub struct ModelBus(pub Shared);

impl Bus for ModelBus {
    fn read32(&self, off: u32) -> u32 {
        self.0.borrow_mut().read(off)
    }

    fn write32(&self, off: u32, val: u32) {
        self.0.borrow_mut().write(off, val)
    }
}
