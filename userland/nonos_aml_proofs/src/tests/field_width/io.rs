// NONOS Operating System (AGPL-3.0-or-later)
//! Between the rig and the model: the accesses a case made, a field's
//! value as a number, and the value to write as AML takes it.

use super::gen::Case;
use super::model::Log;
use crate::object::Object;
use crate::rig::Rig;

pub fn log(r: &Rig, c: &Case, from: usize) -> Log {
    let mine = r.probe.log[from..].iter().filter(|l| l.0 == c.space);
    mine.map(|l| (l.1, l.2, l.3)).collect()
}

pub fn number(o: &Object) -> u128 {
    match o {
        Object::Integer(n) => u128::from(*n),
        Object::Buffer(b) => b.borrow().iter().rev().fold(0, |v, x| v << 8 | u128::from(*x)),
        o => panic!("{o:?}"),
    }
}

pub fn arg(c: &Case) -> Object {
    match c.bits {
        0..=64 => Object::Integer(c.value as u64),
        n => Object::buffer(c.value.to_le_bytes()[..n.div_ceil(8) as usize].to_vec()),
    }
}
