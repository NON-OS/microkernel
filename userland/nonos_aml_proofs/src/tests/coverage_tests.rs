// NONOS Operating System (AGPL-3.0-or-later)
//! The opcode table holds exactly the opcodes of section 20.3: every byte
//! value the catalogue does not use, on both pages, is refused as unknown.

use super::catalog::all;
use crate::enc::*;
use crate::error::ErrorKind;
use crate::flat::parse;
use crate::grammar::is_name_start;

#[test]
fn every_other_opcode_is_refused() {
    let known: Vec<u16> = all().iter().map(|c| c.op.0).chain([0xA1, 0x5B31]).collect();
    for b in 0u16..=0xFF {
        if known.contains(&b) || is_name_start(b as u8) || b == 0x5B {
            continue;
        }
        let err = parse(&[b as u8]).unwrap_err();
        assert_eq!(err.kind, ErrorKind::Opcode(b), "primary {b:#04x}");
    }
    for b in 0u16..=0xFF {
        let code = 0x5B00 | b;
        if known.contains(&code) {
            continue;
        }
        let err = parse(&op(code)).unwrap_err();
        assert_eq!(err.kind, ErrorKind::Opcode(code), "extended {b:#04x}");
    }
}

#[test]
fn the_catalogue_names_each_opcode_once() {
    let mut ops: Vec<u16> = all().iter().map(|c| c.op.0).collect();
    let n = ops.len();
    ops.sort();
    ops.dedup();
    assert_eq!(ops.len(), n);
    // Section 20.3 lists 113 opcodes, the eight locals and seven arguments
    // among them; ElseOp and DebugOp are proved on their own, so 111 cases.
    assert_eq!(n, 111);
}
