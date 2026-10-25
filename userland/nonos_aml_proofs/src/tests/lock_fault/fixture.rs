// NONOS Operating System (AGPL-3.0-or-later)
//! A scene (`gen.rs`) as AML: the Lock field FLD and \M, which reaches it.

use super::gen::{Fault, Scene};
use crate::enc::*;

/// The field FLD, with the Lock rule, as the scene asks.
fn fld(sc: &Scene) -> Vec<u8> {
    let sel_space = if sc.fault == Fault::Select { 3 } else { 1 };
    let past = if sc.fault == Fault::Data { skip(8) } else { Vec::new() };
    match sc.kind {
        0 => cat(&[&region("RG", 1, 0x60, 1), &field("RG", 0x11, &[past, unit("FLD", 8)])]),
        1 => cat(&[
            &region("SLR", sel_space, 0x10, 1),
            &field("SLR", 0x01, &[unit("BNK", 8)]),
            &region("RG", 1, 0x80, 1),
            &pkg(
                0x5B87,
                &cat(&[&name("RG"), &name("BNK"), &int(1), &[0x11], &past, &unit("FLD", 8)]),
            ),
        ]),
        _ => cat(&[
            &region("IXR", sel_space, 0x20, 1),
            &field("IXR", 0x01, &[unit("IDX", 8)]),
            &region("DTR", 1, 0x90, 1),
            &field("DTR", 0x01, &[past, unit("DAT", 8)]),
            &pkg(0x5B86, &cat(&[&name("IDX"), &name("DAT"), &[0x11], &unit("FLD", 8)])),
        ]),
    }
}

pub fn source(sc: &Scene) -> Vec<u8> {
    let gl = |op: u16, rest: &[u8]| e(op, &[&name("\\_GL"), rest]);
    let mut body = Vec::new();
    if sc.outer {
        body.extend(gl(0x5B23, &[0xFF, 0xFF]));
    }
    for i in 0..sc.reps {
        body.extend(match i % 2 {
            0 => store(&int(1), &name("FLD")),
            _ => store(&name("FLD"), &local(0)),
        });
    }
    if sc.outer {
        body.extend(gl(0x5B27, &[]));
    }
    body.extend(ret(&int(0)));
    cat(&[&fld(sc), &method("\\M", 0, &body)])
}
