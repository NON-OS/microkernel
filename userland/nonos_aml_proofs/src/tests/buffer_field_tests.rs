// NONOS Operating System (AGPL-3.0-or-later)
//! Buffer fields: made from a method's argument as _OSC does, read and
//! written in place, bounded by their buffer, gone with their method.

use crate::consts::at;
use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::run::{boot, call, num, raw};

#[test]
fn an_osc_style_capabilities_buffer_is_read_and_written_in_place() {
    // CreateDWordField(Arg0, 4, CDW2); Or(CDW2, 0x10, CDW2); Return(Arg0)
    let body = cat(&[
        &e(0x8A, &[&arg(0), &int(4), &name("CDW2")]),
        &e(0x7D, &[&name("CDW2"), &int(0x10), &name("CDW2")]),
        &e(0x8D, &[&arg(0), &int(0), &name("BIT0")]),
        &store(&int(1), &name("BIT0")),
        &ret(&arg(0)),
    ]);
    let (mut aml, _) = boot(&method("\\_OSC", 1, &body), 2);
    let input = Object::buffer(vec![0, 0, 0, 0, 1, 0, 0, 0]);
    let out = call(&mut aml, "\\_OSC", vec![input]).unwrap();
    assert_eq!(raw(&out), [1, 0, 0, 0, 0x11, 0, 0, 0]);
    assert!(at(&aml, "\\_OSC.CDW2").is_none(), "gone with the method");
}

#[test]
fn create_field_takes_any_bit_span() {
    let defs = name_obj("BUF0", &buffer(12, &[0xFF; 12]));
    let body = cat(&[
        &e(0x5B13, &[&name("BUF0"), &int(4), &int(72), &name("WIDE")]),
        &store(&int(0), &name("WIDE")),
        &ret(&name("BUF0")),
    ]);
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &body)]), 2);
    let out = raw(&call(&mut aml, "\\M", vec![]).unwrap());
    assert_eq!(out[..11], [0x0F, 0, 0, 0, 0, 0, 0, 0, 0, 0xF0, 0xFF]);
}

#[test]
fn a_buffer_field_past_its_buffer_is_a_fault() {
    let body = cat(&[&e(0x8F, &[&arg(0), &int(1), &name("QW")]), &ret(&name("QW"))]);
    let (mut aml, _) = boot(&method("\\M", 1, &body), 2);
    let r = call(&mut aml, "\\M", vec![Object::buffer(vec![0; 8])]);
    assert_eq!(r.unwrap_err().kind, FaultKind::Index);
    let ok = call(&mut aml, "\\M", vec![Object::buffer(vec![1; 9])]);
    assert_eq!(num(&ok.unwrap()), 0x0101_0101_0101_0101);
}

#[test]
fn a_load_time_buffer_field_resolves_on_first_use() {
    let defs = cat(&[
        &name_obj("BUF0", &buffer(4, &[0x34, 0x12])),
        &e(0x8B, &[&name("BUF0"), &int(0), &name("WRD0")]),
    ]);
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &ret(&name("WRD0")))]), 2);
    assert_eq!(num(&call(&mut aml, "\\M", vec![]).unwrap()), 0x1234);
}
