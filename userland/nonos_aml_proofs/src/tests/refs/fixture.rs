// NONOS Operating System (AGPL-3.0-or-later)
//! T1 (\D1: VAL, PKG, BUF, region R); T2 after it (\AV aliasing \D1.VAL,
//! \F2 a field over \D1.R, \BFX a buffer field over \D1.BUF); a filler of
//! 0xF0-marked names for freed slots; and the DSDT, keeping in \PK
//! RefOf(\D1.VAL), Index(\D1.PKG, 1), RefOf(\AV) and a lost local's RefOf.

use crate::enc::*;

pub fn t1() -> Vec<u8> {
    let body = cat(&[
        &name_obj("VAL", &int(0x11)),
        &name_obj("PKG", &package(2, &[int(0x21), int(0x22)])),
        &name_obj("BUF", &buffer(4, &[0x31, 0x32, 0x33, 0x34])),
        &region("R", 1, 0x60, 2),
        &field("R", 1, &[unit("F1", 8)]),
    ]);
    table(b"SSDT", 2, &device("\\D1", &body))
}

pub fn t2() -> Vec<u8> {
    let alias = cat(&[&op(0x06), &name("\\D1.VAL"), &name("\\AV")]);
    let f2 = field("\\D1.R", 1, &[skip(8), unit("F2", 8)]);
    let bfx = e(0x8C, &[&name("\\D1.BUF"), &int(2), &name("\\BFX")]);
    table(b"SSDT", 2, &cat(&[&alias, &f2, &bfx]))
}

pub fn filler() -> Vec<u8> {
    let names: Vec<Vec<u8>> =
        (0..12).map(|i| name_obj(&format!("V{i:02}"), &int(0xF000 + i))).collect();
    table(b"SSDT", 2, &device("\\FL", &names.concat()))
}

pub fn dsdt() -> Vec<u8> {
    let at = |i: u64| e(0x88, &[&name("\\PK"), &int(i), &[0]]);
    let keep = cat(&[
        &store(&e(0x71, &[&name("\\D1.VAL")]), &at(0)),
        &store(&e(0x88, &[&name("\\D1.PKG"), &int(1), &[0]]), &at(1)),
        &store(&e(0x71, &[&name("\\AV")]), &at(2)),
    ]);
    let load = |t: &str, h: &str| {
        cat(&[&e(0x5B20, &[&name(t), &local(0)]), &e(0x9D, &[&local(0), &name(h)])])
    };
    let through = |p: &str| e(0x83, &[&at(u64::from(p.as_bytes()[3] - b'0'))]);
    let read = |p: &str| method(p, 0, &ret(&through(p)));
    let write = |p: &str| method(&p.replace("\\R", "\\W"), 0, &store(&int(0x99), &through(p)));
    cat(&[
        &name_obj("\\TB1", &buffer(t1().len() as u64, &t1())),
        &name_obj("\\TB2", &buffer(t2().len() as u64, &t2())),
        &name_obj("\\TBF", &buffer(filler().len() as u64, &filler())),
        &name_obj("\\H1", &int(0)),
        &name_obj("\\H2", &int(0)),
        &name_obj("\\PK", &package(4, &[int(0), int(0), int(0), int(0)])),
        &method("\\SETU", 0, &cat(&[&load("\\TB1", "\\H1"), &load("\\TB2", "\\H2"), &keep])),
        &method(
            "\\MKLC",
            0,
            &cat(&[&name_obj("LOC", &int(0x44)), &store(&e(0x71, &[&name("LOC")]), &at(3))]),
        ),
        &method("\\UNLD", 0, &e(0x5B2A, &[&name("\\H1")])),
        &method("\\FILL", 0, &e(0x5B20, &[&name("\\TBF"), &local(0)])),
        &method("\\RELD", 0, &e(0x5B20, &[&name("\\TB1"), &local(0)])),
        &method("\\RAV_", 0, &ret(&name("\\AV"))),
        &method("\\WAV_", 0, &store(&int(0x99), &name("\\AV"))),
        &method("\\RF2_", 0, &ret(&name("\\F2"))),
        &method("\\WF2_", 0, &store(&int(0x77), &name("\\F2"))),
        &method("\\RBX_", 0, &ret(&name("\\BFX"))),
        &read("\\RP0"),
        &read("\\RP1"),
        &read("\\RP2"),
        &read("\\RP3"),
        &write("\\RP0"),
        &write("\\RP2"),
        &write("\\RP3"),
    ])
}
