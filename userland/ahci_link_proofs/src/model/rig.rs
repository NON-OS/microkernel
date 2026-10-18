// NONOS Operating System (AGPL-3.0-or-later)
//! A served port on the model HBA: the driver's `Port` over host memory, and
//! the model plugged in for this thread's register accesses until dropped.

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::block::Block;
use super::hba::Hba;
use super::identify::CAPACITY;
use super::plug::{plug, unplug};
use super::script::Step;
use crate::constants::ata::{DATA_BUF_BYTES, STRUCT_REGION_BYTES};
use crate::constants::timing::COMMAND_MS;
use crate::engine::port::Port;
use crate::engine::region::DmaRegion;
use crate::identity::Names;
use crate::regs::Regs;

pub struct Rig {
    pub port: Port,
    pub hba: Rc<RefCell<Hba>>,
    /// A register file nothing reaches while the model is plugged in.
    regs: Block,
}

impl Rig {
    /// A port on an HBA with or without CAP.SCLO, whose drive runs `script`.
    /// Its regions are broker grants, which the model reaches by their bus
    /// addresses (`iommu`).
    pub fn new(script: Vec<Step>, sclo: bool) -> Self {
        let region = |len| DmaRegion::map(1, 3, len, 0).expect("a DMA grant");
        let s = STRUCT_REGION_BYTES;
        let port = Port {
            clb: region(s),
            ctba: region(s),
            _fb: region(s),
            data: region(DATA_BUF_BYTES),
            base: 0,
            capacity_sectors: CAPACITY,
            names: Names::empty(),
            last_tfd: 0,
            wedged: false,
            command_ms: COMMAND_MS,
            reset_window: None,
        };
        let hba = Rc::new(RefCell::new(Hba::new(port.clb.device_addr(), script)));
        hba.borrow_mut().quirks.sclo = sclo;
        plug(hba.clone());
        Self { port, hba, regs: Block::new(s) }
    }

    pub fn regs(&self) -> Regs {
        Regs::new(self.regs.addr())
    }

    pub fn hba(&self) -> RefMut<'_, Hba> {
        self.hba.borrow_mut()
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        unplug();
    }
}
