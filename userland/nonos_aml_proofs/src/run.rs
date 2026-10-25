// NONOS Operating System (AGPL-3.0-or-later)
//! Running methods in the shipping interpreter: load a table, call a method
//! by path, read the result.

use crate::exec::{ExecLimits, Fault, Machine};
use crate::load::Aml;
use crate::object::Object;
pub use crate::probe::Probe;

/// Loads `aml` as a DSDT of `revision`, running its load-time code.
pub fn boot(aml: &[u8], revision: u8) -> (Aml, Probe) {
    let mut a = Aml::default();
    let mut probe = Probe::default();
    let mut m = Machine::new(&mut a, &mut probe, ExecLimits::DEFAULT);
    m.load_table(&mut a, crate::enc::table(b"DSDT", revision, aml)).expect("DSDT loads");
    (a, probe)
}

pub fn call_with(
    aml: &mut Aml,
    probe: &mut Probe,
    limits: ExecLimits,
    path: &str,
    args: Vec<Object>,
) -> Result<Object, Fault> {
    let id = crate::consts::at(aml, path).unwrap_or_else(|| panic!("{path} exists"));
    Machine::new(aml, probe, limits).evaluate(aml, id, args)
}

pub fn call(aml: &mut Aml, path: &str, args: Vec<Object>) -> Result<Object, Fault> {
    call_with(aml, &mut Probe::default(), ExecLimits::DEFAULT, path, args)
}

/// Defines `\M` as a method of `args` arguments around `body`, loads it in a
/// 64-bit table and calls it.
pub fn eval(body: &[u8], args: Vec<Object>) -> Result<Object, Fault> {
    let (mut aml, _) = boot(&crate::enc::method("\\M", args.len() as u8, body), 2);
    call(&mut aml, "\\M", args)
}

pub fn num(o: &Object) -> u64 {
    match o {
        Object::Integer(v) => *v,
        o => panic!("{o:?} is not an integer"),
    }
}

pub fn raw(o: &Object) -> Vec<u8> {
    match o {
        Object::String(b) | Object::Buffer(b) => b.borrow().clone(),
        o => panic!("{o:?} has no bytes"),
    }
}
