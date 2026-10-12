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
use crate::identity::Names;
use crate::regs::Regs;

pub struct Rig {
    pub port: Port,
    pub hba: Rc<RefCell<Hba>>,
    /// Command list, command table, FIS area, data buffer, and a register
    /// file nothing reaches while the model is plugged in.
    pub(super) mem: [Block; 5],
}

impl Rig {
    /// A port on an HBA with or without CAP.SCLO, whose drive runs `script`.
    pub fn new(script: Vec<Step>, sclo: bool) -> Self {
        let s = STRUCT_REGION_BYTES;
        let mem = [s, s, s, DATA_BUF_BYTES, s].map(Block::new);
        let port = Port {
            clb: mem[0].region(),
            ctba: mem[1].region(),
            _fb: mem[2].region(),
            data: mem[3].region(),
            base: 0,
            capacity_sectors: CAPACITY,
            names: Names::empty(),
            last_tfd: 0,
            wedged: false,
            command_ms: COMMAND_MS,
            reset_window: None,
        };
        let hba = Rc::new(RefCell::new(Hba::new(mem[0].addr(), script)));
        hba.borrow_mut().quirks.sclo = sclo;
        plug(hba.clone());
        Self { port, hba, mem }
    }

    pub fn regs(&self) -> Regs {
        Regs::new(self.mem[4].addr())
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
