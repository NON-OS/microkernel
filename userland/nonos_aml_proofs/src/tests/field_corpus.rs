// NONOS Operating System (AGPL-3.0-or-later)
//! Regions, fields, an index field, buffer fields and a lock field, for the
//! field mutation proofs to break.

use crate::enc::*;

pub fn corpus() -> Vec<u8> {
    let regs = cat(&[
        &region("REG0", 1, 0x60, 8),
        &field("REG0", 0x11, &[unit("IDX", 8), unit("DAT", 8), skip(4), unit("BTS", 12)]),
    ]);
    let ifield =
        pkg(0x5B86, &cat(&[&name("IDX"), &name("DAT"), &[0x01], &skip(8), &unit("CFG", 16)]));
    let ec =
        cat(&[&region("ECR", 3, 0, 0x100), &field("ECR", 0x41, &[skip(0x40), unit("ECV", 24)])]);
    cat(&[
        &regs,
        &ifield,
        &ec,
        &method(
            "F1",
            1,
            &cat(&[
                &store(&arg(0), &name("BTS")),
                &store(&name("CFG"), &local(0)),
                &ret(&e(0x72, &[&local(0), &name("ECV"), &[0]])),
            ]),
        ),
        &method(
            "F2",
            1,
            &cat(&[
                &e(0x8A, &[&arg(0), &int(0), &name("DW0")]),
                &store(&int(0x55), &name("DW0")),
                &ret(&arg(0)),
            ]),
        ),
        &method(
            "F3",
            1,
            &cat(&[
                &region("TMP", 0, 0x1000, 0x10),
                &field("TMP", 0x03, &[unit("Q", 64)]),
                &store(&arg(0), &name("Q")),
                &ret(&name("Q")),
            ]),
        ),
    ])
}
