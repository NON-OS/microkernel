// NONOS Operating System (AGPL-3.0-or-later)
//! Named objects, namespace modifiers and statements (sections 20.2.5.1 to
//! 20.2.5.3).

use super::catalog::{case, pkg_case, Case};
use crate::enc::*;

pub fn objects() -> Vec<Case> {
    let (n, m) = (name("OBJ1"), name("\\_SB.OBJ2"));
    let ret = cat(&[&[0xA4], &int(1)]);
    let fl = cat(&[&name("FLD1"), &[0x08]]);
    vec![
        pkg_case(0x10, &[&name("\\_SB"), &ret], 2),
        pkg_case(0x5B82, &[&n, &ret], 2),
        pkg_case(0x5B85, &[&n], 2),
        pkg_case(0x14, &[&n, &[0x01], &ret], 3),
        pkg_case(0x5B84, &[&n, &[0x00], &[0x00, 0x00]], 4),
        pkg_case(0x5B83, &[&n, &[0x01], &[0x10, 0x04, 0, 0], &[0x06]], 5),
        pkg_case(0x5B81, &[&n, &[0x01], &fl], 3),
        pkg_case(0x5B86, &[&n, &m, &[0x01], &fl], 4),
        pkg_case(0x5B87, &[&n, &m, &int(1), &[0x01], &fl], 5),
        case(0x06, &[&n, &m], 2),
        case(0x08, &[&n, &int(0x1234)], 2),
        case(0x15, &[&m, &[0x08, 0x02]], 3),
        case(0x5B80, &[&n, &[0x00], &int(0xFED0_0000), &int(0x400)], 4),
        case(0x5B88, &[&n, &string("SSDT"), &string("NONOS"), &string("T")], 4),
        case(0x5B01, &[&n, &[0x00]], 2),
        case(0x5B02, &[&n], 1),
        case(0x8A, &[&[LOCAL0], &int(0), &n], 3),
        case(0x8B, &[&[LOCAL0], &int(0), &n], 3),
        case(0x8C, &[&[LOCAL0], &int(0), &n], 3),
        case(0x8D, &[&[LOCAL0], &int(0), &n], 3),
        case(0x8F, &[&[LOCAL0], &int(0), &n], 3),
        case(0x5B13, &[&[LOCAL0], &int(0), &int(3), &n], 4),
    ]
}

pub fn statements() -> Vec<Case> {
    let n = name("\\_SB.OBJ2");
    vec![
        pkg_case(0xA0, &[&int(1), &[0xA3]], 2),
        pkg_case(0xA2, &[&int(0), &[0xA5]], 2),
        case(0xA5, &[], 0),
        case(0x9F, &[], 0),
        case(0xA3, &[], 0),
        case(0xCC, &[], 0),
        case(0x86, &[&n, &int(0x80)], 2),
        case(0x5B27, &[&n], 1),
        case(0x5B26, &[&n], 1),
        case(0x5B24, &[&n], 1),
        case(0x5B2A, &[&n], 1),
        case(0xA4, &[&int(1)], 1),
        case(0x5B22, &[&int(10)], 1),
        case(0x5B21, &[&int(10)], 1),
        case(0x5B32, &[&[0x01], &[1, 0, 0, 0], &int(0)], 3),
    ]
}
