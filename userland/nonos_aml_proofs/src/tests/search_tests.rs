// NONOS Operating System (AGPL-3.0-or-later)
//! Name resolution (section 5.3): a lone NameSeg is searched for up the
//! parent chain, every other path is followed exactly, and the predefined
//! root objects exist before any table.

use crate::consts::{at, load};
use crate::enc::*;
use crate::grammar::{name_string, Cursor};
use crate::load::Aml;
use crate::namespace::{Id, Kind, ROOT};

fn resolve(aml: &Aml, scope: Id, path: &str) -> Option<Id> {
    let b = name(path);
    let p = name_string(&mut Cursor::new(&b, 0, b.len() as u32)).unwrap();
    aml.ns.resolve(scope, &b, p)
}

fn tree() -> Aml {
    let ec = device("EC0", &name_obj("ECOK", &int(1)));
    let lpc = device("LPCB", &ec);
    let pci = device("PCI0", &cat(&[&lpc, &name_obj("BUSN", &int(0))]));
    load(&scope("\\_SB", &cat(&[&pci, &name_obj("TOP0", &int(2))])), &[]).0
}

#[test]
fn the_predefined_objects_exist_before_any_table() {
    let aml = Aml::default();
    for (p, kind) in [
        ("\\_GPE", Kind::Scope),
        ("\\_PR", Kind::Scope),
        ("\\_SB", Kind::Device),
        ("\\_SI", Kind::Scope),
        ("\\_TZ", Kind::Scope),
        ("\\_GL", Kind::Mutex),
        ("\\_OS", Kind::Name),
        ("\\_REV", Kind::Name),
    ] {
        assert_eq!(aml.ns.get(at(&aml, p).expect(p)).unwrap().kind, kind, "{p}");
    }
    let osi = aml.ns.get(at(&aml, "\\_OSI").unwrap()).unwrap();
    assert!(matches!(osi.kind, Kind::Method(m) if m.args() == 1));
}

#[test]
fn a_lone_nameseg_is_searched_for_up_the_parent_chain() {
    let aml = tree();
    let ec = at(&aml, "\\_SB.PCI0.LPCB.EC0").unwrap();
    assert_eq!(resolve(&aml, ec, "BUSN"), at(&aml, "\\_SB.PCI0.BUSN"));
    assert_eq!(resolve(&aml, ec, "TOP0"), at(&aml, "\\_SB.TOP0"));
    assert_eq!(resolve(&aml, ec, "_OSI"), at(&aml, "\\_OSI"));
    assert_eq!(resolve(&aml, ec, "NONE"), None);
}

#[test]
fn any_other_path_is_followed_exactly() {
    let aml = tree();
    let ec = at(&aml, "\\_SB.PCI0.LPCB.EC0").unwrap();
    assert_eq!(resolve(&aml, ec, "^^BUSN"), at(&aml, "\\_SB.PCI0.BUSN"));
    assert_eq!(resolve(&aml, ec, "^BUSN"), None, "a parent prefix stops the search");
    assert_eq!(resolve(&aml, ec, "LPCB.EC0"), None, "a two-segment path is not searched");
    let pci = at(&aml, "\\_SB.PCI0").unwrap();
    assert_eq!(resolve(&aml, pci, "LPCB.EC0"), Some(ec));
    assert_eq!(resolve(&aml, ec, "^^^^^^BUSN"), None, "climbing past the root names nothing");
    assert_eq!(resolve(&aml, ec, "\\"), Some(ROOT));
}

#[test]
fn children_keep_the_order_they_were_defined_in() {
    let aml = tree();
    let pci = at(&aml, "\\_SB.PCI0").unwrap();
    let segs: Vec<[u8; 4]> = aml.ns.children(pci).map(|c| aml.ns.get(c).unwrap().seg).collect();
    assert_eq!(segs, [*b"LPCB", *b"BUSN"]);
}
