// NONOS Operating System (AGPL-3.0-or-later)
//! Load, LoadTable and Unload at run time, and a region a method makes from
//! its argument.

use crate::consts::at;
use crate::enc::*;
use crate::object::Object;
use crate::probe::Probe;
use crate::rig::Rig;

#[path = "load_admit_tests.rs"]
mod admit;
#[path = "unload_cut/mod.rs"]
mod unload_cut;

fn ssdt() -> Vec<u8> {
    table(
        b"SSDT",
        2,
        &scope("\\_SB", &cat(&[&name_obj("NEW0", &int(9)), &name_obj("PRM0", &int(0))])),
    )
}

#[test]
fn load_from_a_buffer_then_unload() {
    let defs = name_obj("TBL0", &buffer(ssdt().len() as u64, &ssdt()));
    let body = cat(&[&store(&e(0x5B20, &[&name("TBL0"), &local(0)]), &local(1)), &ret(&local(0))]);
    let un = cat(&[&e(0x5B2A, &[&arg(0)])]);
    let mut r = Rig::new(&cat(&[&defs, &method("\\L", 0, &body), &method("\\U", 1, &un)]));
    let handle = r.call("\\L", vec![]).unwrap();
    assert!(matches!(handle, Object::Table(1)));
    assert_eq!(r.num("\\_SB.NEW0"), 9);
    r.call("\\U", vec![handle]).unwrap();
    assert!(at(&r.aml, "\\_SB.NEW0").is_none());
}

#[test]
fn load_from_a_region_reads_it_a_byte_at_a_time() {
    let t = ssdt();
    let mut probe = Probe::default();
    for (i, b) in t.iter().enumerate() {
        probe.bytes.insert((0, 0x1000 + i as u64), *b);
    }
    let src = cat(&[
        &region("TREG", 0, 0x1000, t.len() as u64),
        &method("\\L", 0, &ret(&e(0x5B20, &[&name("TREG"), &[0]]))),
    ]);
    let mut r = Rig::with(&src, probe).connected();
    assert_eq!(r.num("\\L"), u64::MAX);
    assert_eq!(r.num("\\_SB.NEW0"), 9);
}
