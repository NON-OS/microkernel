// NONOS Operating System (AGPL-3.0-or-later)
//! A name defined twice (section 5.3): the first definition stands, the second
//! is noted; a container defined again still has its TermList loaded into the
//! first. Missing scopes, parents and alias targets are noted, never guessed.

use crate::consts::{at, load};
use crate::enc::*;
use crate::namespace::{Kind, NoteKind, Origin, Predefined};

fn kinds(aml: &crate::load::Aml) -> Vec<NoteKind> {
    aml.notes.list.iter().map(|n| n.kind).collect()
}

#[test]
fn the_first_definition_stands_and_the_second_is_noted() {
    let dsdt = scope("\\_SB", &name_obj("VAL0", &int(1)));
    let ssdt = scope("\\_SB", &method("VAL0", 0, &[0xA3]));
    let (aml, _) = load(&dsdt, &[ssdt]);
    let v = aml.ns.get(at(&aml, "\\_SB.VAL0").unwrap()).unwrap();
    assert_eq!((v.kind, v.origin), (Kind::Name, v.origin));
    assert!(matches!(v.origin, Origin::Table { table: 0, .. }));
    assert_eq!(kinds(&aml), [NoteKind::Redefined]);
    assert_eq!(aml.notes.list[0].table, 1);
}

#[test]
fn a_container_defined_again_keeps_both_bodies() {
    let dsdt = scope("\\_SB", &device("PCI0", &name_obj("ONE0", &int(1))));
    let ssdt = scope("\\_SB", &device("PCI0", &name_obj("TWO0", &int(2))));
    let (aml, _) = load(&dsdt, &[ssdt]);
    assert!(at(&aml, "\\_SB.PCI0.ONE0").is_some());
    assert!(at(&aml, "\\_SB.PCI0.TWO0").is_some());
    assert_eq!(kinds(&aml), [NoteKind::Redefined]);
    let (aml, _) = load(&device("\\_SB", &name_obj("SBV0", &int(1))), &[]);
    assert!(at(&aml, "\\_SB.SBV0").is_some(), "Device(\\_SB) fills the predefined \\_SB");
}

#[test]
fn a_table_cannot_replace_a_predefined_object() {
    let (aml, _) = load(&cat(&[&method("\\_OSI", 1, &[0xA3]), &name_obj("\\_REV", &int(5))]), &[]);
    let osi = aml.ns.get(at(&aml, "\\_OSI").unwrap()).unwrap();
    assert_eq!(osi.origin, Origin::Predefined(Predefined::Osi));
    assert_eq!(kinds(&aml), [NoteKind::Redefined, NoteKind::Redefined]);
}

#[test]
fn missing_scopes_parents_and_alias_targets_are_noted() {
    let aml_src = cat(&[
        &scope("\\_SB.NONE", &device("LOST", &[])),
        &device("\\_SB.NONE.LOST", &[]),
        &cat(&[&[0x06], &name("\\_SB.NONE"), &name("ALI0")]),
    ]);
    let (aml, _) = load(&aml_src, &[]);
    assert_eq!(
        kinds(&aml),
        [NoteKind::ScopeMissing, NoteKind::ParentMissing, NoteKind::AliasMissing]
    );
    assert_eq!(aml.notes.list[0].at, 36);
    assert!(at(&aml, "\\ALI0").is_none());
}
