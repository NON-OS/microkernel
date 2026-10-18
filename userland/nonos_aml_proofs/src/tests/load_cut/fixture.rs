// NONOS Operating System (AGPL-3.0-or-later)
//! A scene as tables: the SSDT "OEM1" its pieces make, and the DSDT with
//! \GATE (1, so the If loads), the table's bytes for Load, and \L, which
//! spins, loads, and says 99. Piece i is named with tag i; an EC device's
//! _REG says 0x100 plus its tag.

use super::gen::{Piece, Scene};
use crate::enc::*;

fn piece(i: usize, p: Piece) -> Vec<u8> {
    let adr = || name_obj("_ADR", &int(i as u64));
    match p {
        Piece::Plain => device(&format!("\\D{i:02}"), &adr()),
        Piece::Gated => {
            if_(&e(0x93, &[&name("\\GATE"), &int(1)]), &device(&format!("\\G{i:02}"), &adr()))
        }
        Piece::Stored(v) => {
            let n = format!("\\V{i:02}");
            cat(&[&name_obj(&n, &int(0)), &store(&int(u64::from(v)), &name(&n))])
        }
        Piece::Ec => {
            let reg = method("_REG", 2, &store(&int(0x100 + i as u64), &op(0x5B31)));
            device(&format!("\\E{i:02}"), &cat(&[&region("ECR", 3, i as u64, 1), &reg]))
        }
        Piece::Into => scope("\\_SB", &name_obj(&format!("S{i:03}"), &int(i as u64))),
    }
}

pub fn ssdt(sc: &Scene) -> Vec<u8> {
    let body: Vec<u8> = sc.pieces.iter().enumerate().flat_map(|(i, p)| piece(i, *p)).collect();
    table(b"OEM1", 2, &body)
}

pub fn dsdt(sc: &Scene) -> Vec<u8> {
    let t = ssdt(sc);
    let less = e(0x95, &[&local(0), &int(u64::from(sc.before))]);
    let spin =
        cat(&[&store(&int(0), &local(0)), &pkg(0xA2, &cat(&[&less, &e(0x75, &[&local(0)])]))]);
    let none = string("");
    let lt = e(0x5B1F, &[&string("OEM1"), &none, &none, &string("\\"), &none, &int(0)]);
    let load = match sc.path {
        1 => store(&lt, &local(1)),
        _ => e(0x5B20, &[&name("\\TB"), &local(1)]),
    };
    let body = cat(&[&spin, &load, &store(&int(99), &op(0x5B31)), &ret(&int(0))]);
    cat(&[
        &name_obj("\\GATE", &int(1)),
        &name_obj("\\TB", &buffer(t.len() as u64, &t)),
        &method("\\L", 0, &body),
    ])
}
