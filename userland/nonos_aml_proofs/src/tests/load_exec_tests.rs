// NONOS Operating System (AGPL-3.0-or-later)
//! Load-time code: an If loads the TermList its predicate chooses, other
//! statements go to the interpreter as the table loads, and a failure is
//! noted without stopping the table.

use crate::consts::{at, load};
use crate::enc::*;
use crate::namespace::NoteKind;
use crate::op::Op;

#[test]
fn an_if_loads_the_branch_its_predicate_chooses() {
    for (pred, yes) in [(int(1), true), (int(0), false), (int(0x80), true)] {
        let src = cat(&[&if_(&pred, &device("DEV1", &[])), &else_(&device("DEV2", &[]))]);
        let (aml, _) = load(&src, &[]);
        assert_eq!(at(&aml, "\\DEV1").is_some(), yes);
        assert_eq!(at(&aml, "\\DEV2").is_some(), !yes);
    }
}

#[test]
fn an_if_in_a_device_loads_into_that_device() {
    let src = device("\\_SB.DEV0", &if_(&int(1), &name_obj("INIF", &int(1))));
    let (aml, _) = load(&src, &[]);
    assert!(at(&aml, "\\_SB.DEV0.INIF").is_some());
}

#[test]
fn a_predicate_that_will_not_evaluate_loads_neither_branch() {
    let src = cat(&[
        &if_(&[LOCAL0], &device("DEV1", &[])),
        &else_(&device("DEV2", &[])),
        &device("DEV3", &[]),
    ]);
    let (aml, _) = load(&src, &[]);
    assert!(at(&aml, "\\DEV1").is_none() && at(&aml, "\\DEV2").is_none());
    assert!(at(&aml, "\\DEV3").is_some(), "the rest of the table still loads");
    assert_eq!(aml.notes.list[0].kind, NoteKind::Exec);
}

#[test]
fn other_statements_run_as_the_table_loads() {
    let src = cat(&[
        &name_obj("VAL0", &int(0)),
        &[0x70],
        &int(5),
        &name("VAL0"),
        &[0x86],
        &name("\\_SB"),
        &int(0x80),
        &op(0x5B32),
        &[0x01, 1, 0, 0, 0],
        &int(0),
        &device("LAST", &[]),
    ]);
    let (aml, exec) = load(&src, &[]);
    assert_eq!(exec.ran, [Op::STORE, Op::NOTIFY, Op::FATAL]);
    assert_eq!(aml.notes.list.len(), 1, "Fatal's failure is noted");
    assert!(at(&aml, "\\LAST").is_some());
}

#[test]
fn an_external_defines_nothing() {
    let (aml, _) = load(&external_method("\\_SB.EXT0", 2), &[]);
    assert!(at(&aml, "\\_SB.EXT0").is_none());
}
