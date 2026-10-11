// NONOS Operating System (AGPL-3.0-or-later)
//! NameString, section 20.2.2.

use crate::enc::name;
use crate::error::ErrorKind;
use crate::grammar::{name_string, Cursor, NamePath};

fn read(bytes: &[u8]) -> Result<(NamePath, u32), ErrorKind> {
    let mut c = Cursor::new(bytes, 0, bytes.len() as u32);
    name_string(&mut c).map(|n| (n, c.pos)).map_err(|e| e.kind)
}

#[test]
fn each_form_reads_whole() {
    let cases: [(&str, bool, u8, u8); 7] = [
        ("\\", true, 0, 0),
        ("^", false, 1, 0),
        ("FOO", false, 0, 1),
        ("\\_SB", true, 0, 1),
        ("^^LNKA", false, 2, 1),
        ("\\_SB.PCI0", true, 0, 2),
        ("\\_SB.PCI0.LPCB.EC0", true, 0, 4),
    ];
    for (s, root, up, count) in cases {
        let bytes = name(s);
        let (n, end) = read(&bytes).expect(s);
        assert_eq!((n.root, n.up, n.count), (root, up, count), "{s}");
        assert_eq!(end as usize, bytes.len(), "{s} consumed whole");
    }
    let bytes = name("\\_SB.PCI0.LPCB");
    let (n, _) = read(&bytes).unwrap();
    assert_eq!(n.seg(&bytes, 2), *b"LPCB");
}

#[test]
fn null_name_is_the_null_path() {
    let (n, end) = read(&[0x00]).unwrap();
    assert!(n.is_null());
    assert_eq!(end, 1);
}

#[test]
fn malformed_names_are_refused() {
    let bad: [&[u8]; 6] = [b"abcd", b"1ABC", b"AB-D", b"\\^ABC", &[0x2F, 0x00], &[0x30]];
    for b in bad {
        assert_eq!(read(b).map(|_| ()), Err(ErrorKind::Name), "{b:?}");
    }
    let climb = vec![b'^'; 256];
    assert_eq!(read(&[&climb[..], b"ABCD"].concat()).map(|_| ()), Err(ErrorKind::Name));
}

#[test]
fn a_name_cut_short_is_truncated() {
    for cut in [&b"AB"[..], &[0x2E, b'A', b'B', b'C', b'D', b'E'], &[0x2F], &[0x2F, 3], b"\\"] {
        assert_eq!(read(cut).map(|_| ()), Err(ErrorKind::Truncated), "{cut:?}");
    }
}
