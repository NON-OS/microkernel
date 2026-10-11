// NONOS Operating System (AGPL-3.0-or-later)
//! Loading tables: the DSDT first, SSDTs after it adding to its scopes, and
//! every named object becoming the kind of entry it declares.

use crate::consts::{at, load};
use crate::enc::*;
use crate::namespace::{Kind, Origin};

#[test]
fn each_named_object_becomes_its_kind() {
    let objs = cat(&[
        &pkg(0x14, &cat(&[&name("MTH0"), &[0x2B], &[0xA3]])),
        &name_obj("NAM0", &int(5)),
        &cat(&[&op(0x5B01), &name("MUT0"), &[0x00]]),
        &cat(&[&op(0x5B02), &name("EVT0")]),
        &cat(&[&op(0x5B80), &name("REG0"), &[0x00], &int(0x1000), &int(0x10)]),
        &cat(&[&op(0x5B88), &name("DRG0"), &string("SSDT"), &string("A"), &string("B")]),
        &pkg(0x5B84, &cat(&[&name("PWR0"), &[0x00], &[0x00, 0x00]])),
        &pkg(0x5B85, &name("TZ00")),
        &pkg(0x5B83, &cat(&[&name("CPU0"), &[0], &[0x10, 0x04, 0, 0], &[6]])),
        &name_obj("BUF0", &buffer(4, &[])),
        &cat(&[&[0x8C], &name("BUF0"), &int(1), &name("BYT0")]),
        &cat(&[&op(0x5B13), &name("BUF0"), &int(3), &int(5), &name("BIT0")]),
    ]);
    let (aml, _) = load(&scope("\\_SB", &objs), &[]);
    let kind = |p: &str| aml.ns.get(at(&aml, p).expect(p)).unwrap().kind;
    assert!(matches!(kind("\\_SB.MTH0"), Kind::Method(m) if m.args() == 3
        && m.serialized() && m.sync_level() == 2));
    assert_eq!(kind("\\_SB.NAM0"), Kind::Name);
    assert_eq!(kind("\\_SB.MUT0"), Kind::Mutex);
    assert_eq!(kind("\\_SB.EVT0"), Kind::Event);
    assert_eq!(kind("\\_SB.REG0"), Kind::OpRegion);
    assert_eq!(kind("\\_SB.DRG0"), Kind::DataRegion);
    assert_eq!(kind("\\_SB.PWR0"), Kind::PowerResource);
    assert_eq!(kind("\\_SB.TZ00"), Kind::ThermalZone);
    assert_eq!(kind("\\_SB.CPU0"), Kind::Processor);
    assert_eq!(kind("\\_SB.BYT0"), Kind::BufferField);
    assert_eq!(kind("\\_SB.BIT0"), Kind::BufferField);
    let m = aml.ns.get(at(&aml, "\\_SB.MTH0").unwrap()).unwrap();
    assert!(matches!(m.origin, Origin::Table { table: 0, .. }));
    assert!(aml.notes.list.is_empty(), "{:?}", aml.notes.list);
}

#[test]
fn an_ssdt_adds_to_the_dsdts_scopes() {
    let dsdt = scope("\\_SB", &device("PCI0", &[]));
    let ssdt = scope("\\_SB.PCI0", &device("XHC", &method("_PS0", 0, &[0xA3])));
    let (aml, _) = load(&dsdt, &[ssdt]);
    let xhc = at(&aml, "\\_SB.PCI0.XHC").unwrap();
    assert_eq!(aml.ns.get(xhc).unwrap().parent, at(&aml, "\\_SB.PCI0").unwrap());
    assert!(at(&aml, "\\_SB.PCI0.XHC._PS0").is_some());
    assert_eq!(aml.tables.len(), 2);
}
