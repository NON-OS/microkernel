// NONOS Operating System (AGPL-3.0-or-later)
//! SMBus-style transactions, GPIO pins through a connection, and a
//! DataTableRegion over the platform's copy of a table.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::probe::Probe;
use crate::rig::Rig;
use crate::run::raw;

#[test]
fn an_smbus_write_returns_the_response_in_its_buffer() {
    let defs = cat(&[
        &region("SMB0", 4, 0x1600, 0x100),
        &field("SMB0", 0x05, &[skip(0x20 * 8), access_as(0x05, 0x06), unit("CMD", 8)]),
    ]);
    let body = cat(&[
        &name_obj("BUFF", &buffer(3, &[0, 1, 0x42])),
        &store(&name("BUFF"), &name("CMD")),
        &ret(&name("BUFF")),
    ]);
    let mut r = Rig::new(&cat(&[&defs, &method("\\M", 0, &body)])).connected();
    let out = r.call("\\M", vec![]).unwrap();
    assert_eq!(raw(&out), [0, 0, 1, 0x42], "status, then what the device got");
    let t = &r.probe.transactions[0];
    assert_eq!((t.0, t.1, t.2), (4, 0x1620, [0x05, 0x06, 0]));
    assert_eq!(t.4, [0, 1, 0x42]);
}

#[test]
fn a_gpio_field_reads_and_writes_its_pins() {
    let conn = cat(&[&[0x02], &name("GCON")]);
    let defs = cat(&[
        &name_obj("GCON", &buffer(4, &[0x8C, 0, 0, 0])),
        &region("GPR", 8, 0, 1),
        &field("GPR", 0x01, &[conn, skip(5), unit("PIN", 2)]),
    ]);
    let body = cat(&[&store(&arg(0), &name("PIN")), &ret(&name("PIN"))]);
    let mut r = Rig::new(&cat(&[&defs, &method("\\M", 1, &body)])).connected();
    assert_eq!(crate::run::num(&r.call("\\M", vec![Object::Integer(7)]).unwrap()), 3);
    assert_eq!(r.probe.pins[&5], 3);
}

#[test]
fn a_data_table_region_reads_the_platforms_copy_and_refuses_writes() {
    let tbl = table(b"OEMT", 2, &[0xA5; 4]);
    let defs = cat(&[
        &op(0x5B88),
        &name("DTR"),
        &string("OEMT"),
        &string(""),
        &string(""),
        &field("DTR", 0x01, &[skip(36 * 8), unit("FST", 8)]),
    ]);
    let src = cat(&[
        &defs,
        &method("\\R", 0, &ret(&name("FST"))),
        &method("\\W", 0, &store(&int(1), &name("FST"))),
    ]);
    let mut r = Rig::with(&src, Probe { tables: vec![tbl], ..Default::default() });
    assert_eq!(r.num("\\R"), 0xA5);
    assert_eq!(r.call("\\W", vec![]).unwrap_err().kind, FaultKind::Region);
}
