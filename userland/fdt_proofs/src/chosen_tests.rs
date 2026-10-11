// NONOS Operating System (AGPL-3.0-or-later)
//! `/chosen` and `/aliases`: refuse what is half there.

use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, text, Builder, END};

fn chosen(props: &[(&str, Vec<u8>)]) -> Vec<u8> {
    let mut b = Builder::default().begin("").begin("aliases");
    b = b.prop("a", &text("b")).prop("u", &text("/uart")).end();
    b = b.begin("uart").end().begin("chosen");
    for (name, value) in props {
        b = b.prop(name, value);
    }
    b.end().end().token(END).finish()
}

fn stdout(value: Vec<u8>) -> Result<Option<Vec<u8>>, FdtError> {
    let blob = chosen(&[("stdout-path", value)]);
    let f = Fdt::from_bytes(&blob).unwrap();
    f.stdout().map(|s| s.map(|(_, o)| o.to_vec()))
}

#[test]
fn stdout_path_must_name_a_node() {
    assert_eq!(stdout(text("u:9600")), Ok(Some(b"9600".to_vec())));
    assert_eq!(stdout(text("/uart")), Ok(Some(Vec::new())));
    assert_eq!(stdout(text("/nope")), Err(FdtError::NoSuchNode));
    assert_eq!(stdout(text("a")), Err(FdtError::NoSuchNode));
    assert_eq!(stdout(b"/uart".to_vec()), Err(FdtError::BadString));
    assert_eq!(stdout(b"/u\0rt\0".to_vec()), Err(FdtError::BadString));
}

fn initrd(props: &[(&str, Vec<u8>)]) -> Result<Option<(u64, u64)>, FdtError> {
    let blob = chosen(props);
    Fdt::from_bytes(&blob).unwrap().initrd()
}

#[test]
fn initrd_needs_both_bounds_in_order() {
    let (s, e) = ("linux,initrd-start", "linux,initrd-end");
    assert_eq!(
        initrd(&[(s, cells(&[0x4800_0000])), (e, cells(&[1, 0]))]),
        Ok(Some((0x4800_0000, 1 << 32)))
    );
    assert_eq!(initrd(&[(s, cells(&[0x10]))]), Err(FdtError::OutOfBounds));
    assert_eq!(initrd(&[(s, cells(&[0x10])), (e, cells(&[0x8]))]), Err(FdtError::OutOfBounds));
    assert_eq!(initrd(&[(s, cells(&[0, 0, 1])), (e, cells(&[2]))]), Err(FdtError::BadCells));
}
