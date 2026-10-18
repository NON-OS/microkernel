// NONOS Operating System (AGPL-3.0-or-later)
//! Table order (section 5.2.11.2) and a table refused whole.

use crate::consts::Consts;
use crate::enc::*;
use crate::error::ErrorKind;
use crate::load::Aml;

#[test]
fn tables_load_in_order_or_not_at_all() {
    let mut exec = Consts::default();
    let mut aml = Aml::default();
    let ssdt = table(b"SSDT", 2, &[]);
    let refused = aml.load_table(ssdt.clone(), &mut exec).unwrap_err();
    assert_eq!(refused.kind, ErrorKind::Signature, "an SSDT before the DSDT");
    assert_eq!(aml.load_table(table(b"DSDT", 2, &[]), &mut exec), Ok(0));
    let again = aml.load_table(table(b"DSDT", 2, &[]), &mut exec).unwrap_err();
    assert_eq!(again.kind, ErrorKind::Signature, "a second DSDT");
    let facp = aml.load_table(table(b"FACP", 2, &[]), &mut exec).unwrap_err();
    assert_eq!(facp.kind, ErrorKind::Signature);
    assert_eq!(aml.load_table(ssdt, &mut exec), Ok(1));
    assert_eq!(aml.load_table(table(b"PSDT", 2, &[]), &mut exec), Ok(2));
}

#[test]
fn a_table_that_will_not_parse_adds_nothing() {
    let mut exec = Consts::default();
    let mut aml = Aml::default();
    aml.load_table(table(b"DSDT", 2, &[]), &mut exec).unwrap();
    let before = aml.ns.len();
    let bad = cat(&[&device("GOOD", &[]), &[0x5B, 0xFE]]);
    assert!(aml.load_table(table(b"SSDT", 2, &bad), &mut exec).is_err());
    assert_eq!((aml.ns.len(), aml.tables.len()), (before, 1));
}
