// NONOS Operating System (AGPL-3.0-or-later)
//! \_S5 is read out of AML in every encoding real firmware uses.

use crate::arch::x86_64::acpi::aml::sleep_obj::*;

const S5: [u8; 4] = *b"_S5_";

fn name_pkg(prefix: &[u8], elems: &[u8], count: u8) -> alloc::vec::Vec<u8> {
    let mut v = alloc::vec![0x08u8];
    v.extend_from_slice(prefix);
    v.extend_from_slice(&S5);
    v.push(0x12);
    v.push((2 + elems.len()) as u8);
    v.push(count);
    v.extend_from_slice(elems);
    v
}

#[test]
fn the_common_intel_form_reads_seven_seven() {
    // Name (_S5, Package (0x04) { 0x07, 0x07, Zero, Zero })
    let aml = name_pkg(b"", &[0x0A, 0x07, 0x0A, 0x07, 0x00, 0x00], 4);
    assert_eq!(find_sleep_package(&aml, 5), Some(SleepPackage { slp_typ_a: 7, slp_typ_b: 7 }));
}

#[test]
fn a_root_prefixed_name_and_word_constants_are_read() {
    // Name (\_S5, Package () { 0x0005, One })
    let aml = name_pkg(b"\\", &[0x0B, 0x05, 0x00, 0x01], 2);
    assert_eq!(find_sleep_package(&aml, 5), Some(SleepPackage { slp_typ_a: 5, slp_typ_b: 1 }));
}

#[test]
fn qemu_s5_is_zero_zero() {
    let aml = name_pkg(b"", &[0x00, 0x00, 0x00, 0x00], 4);
    assert_eq!(find_sleep_package(&aml, 5), Some(SleepPackage { slp_typ_a: 0, slp_typ_b: 0 }));
}

#[test]
fn a_single_element_package_packs_both_values() {
    let aml = name_pkg(b"", &[0x0B, 0x07, 0x05], 1);
    assert_eq!(find_sleep_package(&aml, 5), Some(SleepPackage { slp_typ_a: 7, slp_typ_b: 5 }));
}

#[test]
fn a_method_returning_a_package_is_read() {
    // Method (_S5, 0) { Return (Package () { 0x07, 0x07 }) }
    let body = [0x00u8, 0xA4, 0x12, 0x06, 0x02, 0x0A, 0x07, 0x0A, 0x07];
    let mut aml = alloc::vec![0x14u8, (1 + 4 + body.len()) as u8];
    aml.extend_from_slice(&S5);
    aml.extend_from_slice(&body);
    assert_eq!(find_sleep_package(&aml, 5), Some(SleepPackage { slp_typ_a: 7, slp_typ_b: 7 }));
}

#[test]
fn other_states_and_stray_name_bytes_are_not_confused_with_s5() {
    let mut aml = name_pkg(b"", &[0x0A, 0x05, 0x0A, 0x05], 2);
    aml[3] = b'3';
    assert_eq!(find_sleep_package(&aml, 5), None);
    assert_eq!(find_sleep_package(&aml, 3), Some(SleepPackage { slp_typ_a: 5, slp_typ_b: 5 }));
    // "_S5_" inside a string, not a Name: ignored.
    let stray = [0x0D, b'_', b'S', b'5', b'_', 0x00, 0x12, 0x04, 0x02, 0x01, 0x01];
    assert_eq!(find_sleep_package(&stray, 5), None);
}

#[test]
fn elements_that_need_an_interpreter_give_no_answer() {
    // Package { SS5A, SS5B }: names, not constants.
    let aml = name_pkg(b"", b"SS5ASS5B", 2);
    assert_eq!(find_sleep_package(&aml, 5), None);
}

#[test]
fn a_truncated_package_is_refused() {
    let mut aml = name_pkg(b"", &[0x0A, 0x07, 0x0A, 0x07], 2);
    aml.truncate(aml.len() - 1);
    assert_eq!(find_sleep_package(&aml, 5), None);
}

#[test]
fn the_dsdt_wins_over_an_ssdt() {
    let dsdt = name_pkg(b"", &[0x0A, 0x07, 0x0A, 0x07], 2);
    let ssdt = name_pkg(b"", &[0x0A, 0x05, 0x0A, 0x05], 2);
    let empty: &[u8] = &[];
    let got = find_in_blocks([empty, dsdt.as_slice(), ssdt.as_slice()], 5);
    assert_eq!(got, Some(SleepPackage { slp_typ_a: 7, slp_typ_b: 7 }));
}
