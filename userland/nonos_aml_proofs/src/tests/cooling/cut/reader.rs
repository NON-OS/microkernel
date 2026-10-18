// NONOS Operating System (AGPL-3.0-or-later)
//! An evaluator held to a step budget that records, for each object it
//! ran, whether the run finished and with what, and the model's belief.

use crate::exec::{ExecLimits, Fault, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::probe::Probe;

/// Each run: the object, and its integer answer when it finished with one.
pub type Log = Vec<(Id, Option<Option<u64>>)>;

pub fn limited<'a>(
    probe: &'a mut Probe,
    steps: u64,
    log: &'a mut Log,
) -> impl FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + 'a {
    move |aml: &mut Aml, id: Id, args: Vec<Object>| {
        let lim = ExecLimits { steps, ..ExecLimits::DEFAULT };
        let r = Machine::new(aml, probe, lim).evaluate(aml, id, args);
        let int = |o: &Object| if let Object::Integer(v) = o { Some(*v) } else { None };
        log.push((id, r.as_ref().ok().map(int)));
        r
    }
}

/// The model's belief for an answer: in m°C when it finished with an
/// integer from 2232 to 4732 tenths of a kelvin.
pub fn believed(run: Option<Option<u64>>) -> Option<i32> {
    let v = run.flatten().filter(|v| (2232..=4732).contains(v))?;
    Some(v as i32 * 100 - 273_150)
}

pub fn held(aml: &Aml) -> bool {
    !aml.rt.mutexes.is_empty() || aml.rt.global_lock != 0
}
