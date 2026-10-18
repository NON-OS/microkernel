// NONOS Operating System (AGPL-3.0-or-later)
//! LoadTable, and a region a method makes from its argument.

use crate::enc::*;
use crate::object::Object;
use crate::probe::Probe;
use crate::rig::Rig;

fn ssdt() -> Vec<u8> {
    table(
        b"SSDT",
        2,
        &scope("\\_SB", &cat(&[&name_obj("NEW0", &int(9)), &name_obj("PRM0", &int(0))])),
    )
}

#[test]
fn load_table_finds_the_table_and_stores_its_parameter() {
    let lt = e(
        0x5B1F,
        &[
            &string("SSDT"),
            &string(""),
            &string(""),
            &string("\\"),
            &string("\\_SB.PRM0"),
            &int(0x44),
        ],
    );
    let src = method("\\L", 0, &ret(&lt));
    let mut r = Rig::with(&src, Probe { tables: vec![ssdt()], ..Default::default() });
    assert!(matches!(r.call("\\L", vec![]).unwrap(), Object::Table(1)));
    assert_eq!(r.num("\\_SB.PRM0"), 0x44);
    let missing =
        e(0x5B1F, &[&string("XXXX"), &string(""), &string(""), &string(""), &string(""), &int(0)]);
    let mut r = Rig::new(&method("\\L", 0, &ret(&missing)));
    assert_eq!(r.num("\\L"), 0);
}

#[test]
fn a_method_region_built_from_its_argument() {
    // OperationRegion(TMPR, SystemIO, Arg0, 2): the base is the argument.
    let reg = cat(&[&op(0x5B80), &name("TMPR"), &[1], &arg(0), &int(2)]);
    let body = cat(&[
        &reg,
        &field("TMPR", 1, &[unit("LO", 8), unit("HI", 8)]),
        &store(&arg(0), &name("HI")),
        &ret(&name("HI")),
    ]);
    let mut r = Rig::new(&method("\\M", 1, &body)).connected();
    assert_eq!(crate::run::num(&r.call("\\M", vec![Object::Integer(0x80)]).unwrap()), 0x80);
    assert_eq!(r.probe.log[0], (1, 0x81, 1, Some(0x80)));
    assert!(r.aml.rt.regions.is_empty(), "the method's region is gone with it");
}
