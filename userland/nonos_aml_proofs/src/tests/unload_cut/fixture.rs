// NONOS Operating System (AGPL-3.0-or-later)
//! A scene as tables. The SSDT: \D1 with a mutex, an event, an I/O field
//! and a _REG. The DSDT: the SSDT's bytes; \R1 and \R2 for what a reloaded
//! mutex and event answer; \M, which loads, uses, unloads and maybe
//! reloads, saying 1 to 4 at each stage, and last, if it kept a reference
//! to \D1.F1 in Local2, says 5 and reads through it; and \TRY, which takes
//! \D1's mutex with no wait.

use super::gen::{Scene, Use};
use crate::enc::*;

pub fn ssdt() -> Vec<u8> {
    let body = cat(&[
        &cat(&[&op(0x5B01), &name("MTX"), &[15]]),
        &cat(&[&op(0x5B02), &name("EVT")]),
        &region("R", 1, 0x60, 1),
        &field("R", 0x01, &[unit("F1", 8)]),
        &method("_REG", 2, &store(&int(7), &op(0x5B31))),
    ]);
    table(b"SSDT", 2, &device("\\D1", &body))
}

fn use_of(u: Use) -> Vec<u8> {
    match u {
        Use::Acquire => e(0x5B23, &[&name("\\D1.MTX"), &[0xFF, 0xFF]]),
        Use::Signal => e(0x5B24, &[&name("\\D1.EVT")]),
        Use::Notify => e(0x86, &[&name("\\D1"), &int(0x80)]),
        Use::Write => store(&int(5), &name("\\D1.F1")),
        Use::Keep => store(&e(0x71, &[&name("\\D1.F1")]), &local(2)),
    }
}

pub fn dsdt(sc: &Scene) -> Vec<u8> {
    let t = ssdt();
    let say = |n: u64| store(&int(n), &op(0x5B31));
    let load = e(0x5B20, &[&name("\\TB"), &local(1)]);
    let mut body = cat(&[&load, &say(1)]);
    sc.uses.iter().for_each(|u| body.extend(use_of(*u)));
    // A level 1 mutex after the unload: refused if \D1.MTX (level 15),
    // gone with the table, were still counted held.
    let low = cat(&[&e(0x5B23, &[&name("\\LOW"), &[0xFF, 0xFF]]), &e(0x5B27, &[&name("\\LOW")])]);
    body.extend(cat(&[&say(2), &e(0x5B2A, &[&local(1)]), &low, &say(3)]));
    if sc.reload {
        let fresh = cat(&[
            &store(&e(0x5B23, &[&name("\\D1.MTX"), &[0, 0]]), &name("\\R1")),
            &store(&e(0x5B25, &[&name("\\D1.EVT"), &int(0)]), &name("\\R2")),
        ]);
        body.extend(cat(&[&load, &fresh, &say(4)]));
    }
    let keep = sc.uses.iter().any(|u| matches!(u, Use::Keep));
    if keep {
        body.extend(cat(&[&say(5), &store(&e(0x83, &[&local(2)]), &name("\\R1"))]));
    }
    if sc.reload {
        // Taken once since the reload: a second Release finds it not held.
        let rel = e(0x5B27, &[&name("\\D1.MTX")]);
        body.extend(cat(&[&say(6), &rel, &rel]));
    }
    body.extend(ret(&int(0)));
    cat(&[
        &name_obj("\\TB", &buffer(t.len() as u64, &t)),
        &cat(&[&op(0x5B01), &name("\\LOW"), &[1]]),
        &name_obj("\\R1", &int(9)),
        &name_obj("\\R2", &int(9)),
        &method("\\M", 0, &body),
        &method("\\TRY", 0, &ret(&e(0x5B23, &[&name("\\D1.MTX"), &[0, 0]]))),
    ])
}
