// NONOS Operating System (AGPL-3.0-or-later)
//! Aliases, and the bound on the notes a table can leave.

use crate::consts::{at, load};
use crate::enc::*;

#[test]
fn an_alias_resolves_to_what_it_names() {
    let src =
        cat(&[&device("\\_SB.PCI0", &[]), &cat(&[&[0x06], &name("\\_SB.PCI0"), &name("\\PCIA")])]);
    let (aml, _) = load(&src, &[]);
    assert_eq!(at(&aml, "\\PCIA"), at(&aml, "\\_SB.PCI0"));
}

#[test]
fn notes_are_bounded() {
    let dup: Vec<u8> = (0..300).flat_map(|_| name_obj("DUP0", &int(1))).collect();
    let (aml, _) = load(&dup, &[]);
    assert_eq!(aml.notes.list.len(), crate::namespace::Notes::CAP);
    assert_eq!(aml.notes.dropped, 299 - 256);
}
