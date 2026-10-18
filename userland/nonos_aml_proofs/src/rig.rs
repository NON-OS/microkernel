// NONOS Operating System (AGPL-3.0-or-later)
//! A loaded table and the modelled hardware it runs against, kept together
//! so a test can call methods and then look at what the hardware saw.

use crate::exec::{ExecLimits, Fault, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::probe::Probe;
use crate::region::Space;

pub struct Rig {
    pub aml: Aml,
    pub probe: Probe,
}

impl Rig {
    pub fn new(src: &[u8]) -> Self {
        Self::with(src, Probe::default())
    }

    pub fn with(src: &[u8], mut probe: Probe) -> Self {
        let mut aml = Aml::default();
        let mut m = Machine::new(&mut aml, &mut probe, ExecLimits::DEFAULT);
        m.load_table(&mut aml, crate::enc::table(b"DSDT", 2, src)).expect("DSDT loads");
        Rig { aml, probe }
    }

    pub fn connect(&mut self, space: u8) -> Vec<(Id, Fault)> {
        Machine::new(&mut self.aml, &mut self.probe, ExecLimits::DEFAULT)
            .connect(&mut self.aml, Space(space))
    }

    /// Every space up to PCC.
    pub fn connected(mut self) -> Self {
        for s in 0..=0x0A {
            assert!(self.connect(s).is_empty());
        }
        self
    }

    pub fn call(&mut self, path: &str, args: Vec<Object>) -> Result<Object, Fault> {
        let id = crate::consts::at(&self.aml, path).unwrap_or_else(|| panic!("{path} exists"));
        Machine::new(&mut self.aml, &mut self.probe, ExecLimits::DEFAULT).evaluate(
            &mut self.aml,
            id,
            args,
        )
    }

    pub fn num(&mut self, path: &str) -> u64 {
        crate::run::num(&self.call(path, vec![]).expect("evaluates"))
    }
}
