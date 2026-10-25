// NONOS Operating System (AGPL-3.0-or-later)
//! A seeded firmware `_DSM` as AML (`dsm_gen.rs` says what it does).

use super::dsm_gen::{Form, Fw};
use crate::enc::*;

fn value(f: &Form) -> Vec<u8> {
    match f {
        Form::Buf(b) => buffer(b.len() as u64, b),
        Form::Int(n) => int(*n),
        Form::Str => string("1"),
        Form::Pkg => package(1, &[int(1)]),
        Form::Absent => unreachable!(),
    }
}

pub fn source(fw: &Fw) -> Vec<u8> {
    let eq = |a: &[u8], b: &[u8]| e(0x93, &[a, b]);
    let said = store(&e(0x7D, &[&e(0x79, &[&arg(1), &int(8), &[0]]), &arg(2), &[0]]), &op(0x5B31));
    let mut query: Vec<u8> = Vec::new();
    for (rev, f) in fw.forms.iter().enumerate().filter(|(_, f)| !matches!(f, Form::Absent)) {
        query.extend(if_(&eq(&arg(1), &int(rev as u64)), &ret(&value(f))));
    }
    query.extend(ret(&buffer(1, &[0])));
    let served =
        cat(&[&if_(&eq(&arg(2), &int(0)), &query), &ret(&e(0x72, &[&int(0x100), &arg(2), &[0]]))]);
    let uuid = buffer(fw.uuid.len() as u64, &fw.uuid);
    let body = cat(&[&said, &if_(&eq(&arg(0), &uuid), &served), &ret(&buffer(1, &[0]))]);
    cat(&[&device("\\_SB.DEV0", &name_obj("_ADR", &int(0))), &method("\\_SB.DEV0._DSM", 4, &body)])
}
