// NONOS Operating System (AGPL-3.0-or-later)
//! The register-level view the driver's `Mmio` sees.

use std::cell::RefCell;
use std::rc::Rc;

use super::sim::Sim;

#[derive(Clone)]
pub struct SimBus(pub Rc<RefCell<Sim>>);
