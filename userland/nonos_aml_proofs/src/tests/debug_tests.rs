// NONOS Operating System (AGPL-3.0-or-later)
//! A Store to the Debug object.

use crate::enc::*;
use crate::run::{boot, Probe};

#[test]
fn a_store_to_debug_reaches_the_host() {
    let (mut aml, _) = boot(&method("\\M", 0, &store(&string("hi"), &op(0x5B31))), 2);
    let mut probe = Probe::default();
    let id = crate::consts::at(&aml, "\\M").unwrap();
    let mut m = crate::exec::Machine::new(&mut aml, &mut probe, Default::default());
    m.evaluate(&mut aml, id, vec![]).unwrap();
    assert_eq!(probe.debug.len(), 1);
    assert!(probe.debug[0].contains("String"));
}
