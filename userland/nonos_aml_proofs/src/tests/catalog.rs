// NONOS Operating System (AGPL-3.0-or-later)
//! One encoded instance of every object chapter 20's grammar defines, with the
//! number of operands its node must hold. Each is a valid TermObj on its own,
//! so the instances can be laid side by side in one term list.

use super::{catalog_exprs, catalog_objects};
use crate::enc::*;
use crate::op::Op;

pub struct Case {
    pub op: Op,
    pub bytes: Vec<u8>,
    pub kids: usize,
}

pub fn case(code: u16, parts: &[&[u8]], kids: usize) -> Case {
    Case { op: Op(code), bytes: cat(&[&op(code), &parts.concat()]), kids }
}

pub fn pkg_case(code: u16, parts: &[&[u8]], kids: usize) -> Case {
    Case { op: Op(code), bytes: pkg(code, &parts.concat()), kids }
}

/// The leaves: constants, strings, Revision, locals and arguments.
fn leaves() -> Vec<Case> {
    let mut v = vec![
        case(0x00, &[], 0),
        case(0x01, &[], 0),
        case(0xFF, &[], 0),
        case(0x0A, &[&[0x12]], 0),
        case(0x0B, &[&[0x34, 0x12]], 0),
        case(0x0C, &[&[0x78, 0x56, 0x34, 0x12]], 0),
        case(0x0E, &[&[1, 2, 3, 4, 5, 6, 7, 8]], 0),
        case(0x0D, &[b"NONOS\0"], 0),
        case(0x5B30, &[], 0),
    ];
    v.extend((0x60..=0x6E).map(|c| case(c, &[], 0)));
    v
}

/// Every case, objects first.
pub fn all() -> Vec<Case> {
    let mut v = catalog_objects::objects();
    v.extend(catalog_objects::statements());
    v.extend(catalog_exprs::exprs());
    v.extend(catalog_exprs::data());
    v.extend(leaves());
    v
}

/// The cases laid end to end, with the offset each starts at in the AML.
pub fn laid_out() -> (Vec<u8>, Vec<(usize, Case)>) {
    let (mut aml, mut at) = (Vec::new(), Vec::new());
    for c in all() {
        at.push((aml.len(), Case { op: c.op, bytes: c.bytes.clone(), kids: c.kids }));
        aml.extend_from_slice(&c.bytes);
    }
    (aml, at)
}
