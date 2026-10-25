// NONOS Operating System (AGPL-3.0-or-later)
//! The v1 interface: the version line, the two forms `audit` accepts and
//! every other one refused with the usage, and the heading's columns.

use crate::command::builtin::audit::format::{parse, printable, Mode, FORMAT, HEADING, USAGE};

#[test]
fn only_the_two_forms_are_accepted() {
    assert_eq!(FORMAT, b"audit v1");
    assert_eq!(parse(&[]), Ok(Mode::Table));
    assert_eq!(parse(&[b"--agent", b"acpi"]), Ok(Mode::Agent(b"acpi")));
    for bad in [&[&b"--agent"[..]][..], &[b"--agent", b""], &[b"--agent", b"a", b"b"], &[b"-x"]] {
        let e = parse(bad).unwrap_err();
        assert!(e.ends_with(USAGE), "{bad:?}");
    }
    let e = parse(&[b"\x1b[2J"]).unwrap_err();
    assert!(e.iter().all(|b| (0x20..0x7f).contains(b)), "a refused word is printed safe");
}

#[test]
fn printable_keeps_ascii_and_replaces_the_rest() {
    let all: Vec<u8> = (0..=255).collect();
    let p = printable(&all);
    assert_eq!(p.len(), 256);
    for (b, q) in all.iter().zip(&p) {
        assert_eq!(*q, if (0x20..0x7f).contains(b) { *b } else { b'?' });
    }
    let cols = ["pid", "name", "network", "storage", "devices*", "signed by", "capabilities"];
    let h = String::from_utf8(HEADING[0].to_vec()).unwrap();
    let at: Vec<usize> = cols.iter().map(|c| h.find(c).unwrap()).collect();
    assert_eq!(at, [2, 7, 29, 38, 47, 57, 68], "v1's columns");
}
