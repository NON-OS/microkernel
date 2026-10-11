// NONOS Operating System (AGPL-3.0-or-later)
//! Expressions and the data objects with operands (sections 20.2.3 and
//! 20.2.5.4).

use super::catalog::{case, pkg_case, Case};
use crate::enc::*;

pub fn exprs() -> Vec<Case> {
    let (one, l0, t, n) = (int(1), [LOCAL0], [NULL], name("\\_SB.OBJ2"));
    let mut v: Vec<Case> = [0x72, 0x73, 0x74, 0x77, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F]
        .iter()
        .chain(&[0x84, 0x85, 0x88])
        .map(|c| case(*c, &[&one, &one, &t], 3))
        .collect();
    v.push(case(0x78, &[&one, &one, &t, &l0], 4));
    for c in [0x80, 0x81, 0x82, 0x96, 0x97, 0x98, 0x99, 0x5B28, 0x5B29] {
        v.push(case(c, &[&one, &t], 2));
    }
    v.push(case(0x9C, &[&one, &one, &t], 3));
    v.push(case(0x9E, &[&string("ab"), &one, &one, &l0], 4));
    for c in [0x90, 0x91, 0x93, 0x94, 0x95] {
        v.push(case(c, &[&one, &one], 2));
    }
    v.push(case(0x92, &[&one], 1));
    v.push(case(0x83, &[&l0], 1));
    v.push(case(0x70, &[&one, &l0], 2));
    v.push(case(0x9D, &[&one, &l0], 2));
    for c in [0x75, 0x76, 0x71, 0x87, 0x8E] {
        v.push(case(c, &[&l0], 1));
    }
    v.push(case(0x5B12, &[&n, &t], 2));
    v.push(case(0x5B23, &[&n, &[0xFF, 0xFF]], 2));
    v.push(case(0x5B25, &[&n, &one], 2));
    v.push(case(0x89, &[&l0, &[0x01], &one, &[0x00], &one, &int(0)], 6));
    v.push(case(0x5B20, &[&n, &t], 2));
    let s = string("x");
    v.push(case(0x5B1F, &[&s, &s, &s, &s, &s, &s], 6));
    v.push(case(0x5B33, &[], 0));
    v
}

pub fn data() -> Vec<Case> {
    vec![
        pkg_case(0x11, &[&int(3), &[1, 2, 3]], 2),
        pkg_case(0x12, &[&[0x02], &int(1), &name("\\_SB.OBJ2")], 2),
        pkg_case(0x13, &[&int(2), &int(1)], 2),
    ]
}
