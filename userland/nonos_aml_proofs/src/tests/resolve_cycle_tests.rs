// NONOS Operating System (AGPL-3.0-or-later)
//! Buffer fields and regions resolved on first use whose operands name
//! themselves, each other, or a long chain of unresolved objects: each ends
//! with a CallDepth fault inside the stack share, not a stack overflow. The
//! first was found by the `aml` fuzz target (an ASan stack overflow through
//! read_named, buffer_field_in, make_buffer_field and back).

use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{boot, call, num};

fn seg(i: usize) -> String {
    const D: &[u8; 36] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let s = [b'F', D[i / 1296 % 36], D[i / 36 % 36], D[i % 36]];
    String::from_utf8(s.to_vec()).unwrap()
}

#[test]
fn a_buffer_field_made_from_itself_is_a_fault() {
    // CreateByteField(FLD0, Zero, FLD0)
    let defs = e(0x8C, &[&name("FLD0"), &int(0), &name("FLD0")]);
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &ret(&name("FLD0")))]), 2);
    let r = call(&mut aml, "\\M", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::CallDepth);
}

#[test]
fn two_buffer_fields_made_from_each_other_are_a_fault() {
    let defs = cat(&[
        &e(0x8C, &[&name("FLDB"), &int(0), &name("FLDA")]),
        &e(0x8C, &[&name("FLDA"), &int(0), &name("FLDB")]),
    ]);
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &ret(&name("FLDA")))]), 2);
    let r = call(&mut aml, "\\M", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::CallDepth);
}

#[test]
fn a_chain_of_unresolved_buffer_fields_longer_than_the_stack_is_a_fault() {
    // F000 from F001 from ... from F(n-1), none a cycle; the last is
    // made from a buffer, so only the depth stops it.
    let n = 4000;
    let mut defs = name_obj("BUF0", &buffer(8, &[0x5A; 8]));
    defs.extend(e(0x8C, &[&name("BUF0"), &int(0), &name(&seg(n - 1))]));
    for i in (0..n - 1).rev() {
        defs.extend(e(0x8C, &[&name(&seg(i + 1)), &int(0), &name(&seg(i))]));
    }
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &ret(&name("F000")))]), 2);
    let r = call(&mut aml, "\\M", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::CallDepth);
}

#[test]
fn a_short_chain_is_followed_to_its_end_not_cut_by_the_bound() {
    // F001 resolves from BUF0 to an Integer; F000 made from it is a Type
    // fault, as any Create from a non-buffer is. F001 alone still reads.
    let defs = cat(&[
        &name_obj("BUF0", &buffer(4, &[0x34, 0x12])),
        &e(0x8C, &[&name("BUF0"), &int(0), &name("F001")]),
        &e(0x8C, &[&name("F001"), &int(0), &name("F000")]),
    ]);
    let m = cat(&[&method("\\M", 0, &ret(&name("F000"))), &method("\\N", 0, &ret(&name("F001")))]);
    let (mut aml, _) = boot(&cat(&[&defs, &m]), 2);
    assert_eq!(call(&mut aml, "\\M", vec![]).unwrap_err().kind, FaultKind::Type);
    assert_eq!(num(&call(&mut aml, "\\N", vec![]).unwrap()), 0x34);
}

#[test]
fn a_region_whose_offset_reads_its_own_field_is_a_fault() {
    // OperationRegion(REG0, SystemMemory, FU00, 4); Field(REG0) { FU00, 32 }
    let defs = cat(&[
        &op(0x5B80),
        &name("REG0"),
        &[0x00],
        &name("FU00"),
        &int(4),
        &field("REG0", 0x03, &[unit("FU00", 32)]),
    ]);
    let (mut aml, _) = boot(&cat(&[&defs, &method("\\M", 0, &ret(&name("FU00")))]), 2);
    let r = call(&mut aml, "\\M", vec![]);
    assert_eq!(r.unwrap_err().kind, FaultKind::CallDepth);
}
