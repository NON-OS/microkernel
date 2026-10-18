// NONOS Operating System (AGPL-3.0-or-later)
//! A table of methods that exercise most of the interpreter, for the
//! mutation proofs to break.

use crate::enc::*;

pub fn corpus() -> Vec<u8> {
    let idx = e(0x88, &[&name("PKG0"), &local(1), &local(2)]);
    let loopy = pkg(
        0xA2,
        &cat(&[
            &e(0x95, &[&local(1), &int(3)]),
            &idx,
            &e(0x75, &[&local(1)]),
            &store(&e(0x83, &[&local(2)]), &local(3)),
        ]),
    );
    cat(&[
        &name_obj("PKG0", &package(3, &[int(1), string("ab"), buffer(2, &[1, 2])])),
        &name_obj("BUF0", &buffer(4, &[1, 2, 3, 4])),
        &cat(&[&op(0x5B01), &name("MUT0"), &[1]]),
        &cat(&[&op(0x5B02), &name("EVT0")]),
        &device("DEV0", &method("_STA", 0, &ret(&int(0x0F)))),
        &method(
            "M1",
            2,
            &cat(&[
                &store(&e(0x72, &[&arg(0), &arg(1), &[0]]), &local(0)),
                &ret(&e(0x73, &[&string("x"), &local(0), &[0]])),
            ]),
        ),
        &method("M2", 0, &cat(&[&store(&int(0), &local(1)), &loopy, &ret(&local(3))])),
        &method(
            "M3",
            1,
            &cat(&[
                &e(0x5B23, &[&name("MUT0"), &[5, 0]]),
                &e(0x86, &[&name("DEV0"), &int(0x80)]),
                &e(0x5B27, &[&name("MUT0")]),
                &ret(&e(0x9E, &[&name("BUF0"), &arg(0), &int(2), &[0]])),
            ]),
        ),
        &method(
            "M4",
            0,
            &cat(&[
                &name_obj("TMP0", &package(1, &[name("DEV0")])),
                &store(&e(0x71, &[&name("TMP0")]), &local(0)),
                &ret(&e(0x8E, &[&local(0)])),
            ]),
        ),
        &method(
            "M5",
            1,
            &cat(&[
                &if_(&arg(0), &ret(&cat(&[&name("M1"), &arg(0), &int(1)]))),
                &else_(&ret(&name("M2"))),
            ]),
        ),
    ])
}
