// NONOS Operating System (AGPL-3.0-or-later)
//! Reservations: losing one would hand out memory firmware still owns, so
//! every way of losing one is an error.

use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{cells, Builder, END};

#[test]
fn a_reservation_map_without_terminator_is_reported() {
    let mut b = Builder::default().begin("").end().token(END);
    b.rsv = vec![(0x1000, 0x1000)];
    b.rsv_unterminated = true;
    let blob = b.finish();
    let f = Fdt::from_bytes(&blob).unwrap();
    let got: Vec<_> = f.reservations().collect();
    assert_eq!(got, [Ok((0x1000, 0x1000)), Err(FdtError::MissingEnd)]);
}

#[test]
fn reserved_memory_that_does_not_fit_is_an_error() {
    let reg = cells(&[0, 0x1000, 0x1000, 0, 0x8000, 0x1000]);
    let blob = Builder::default()
        .begin("")
        .begin("reserved-memory")
        .prop("ranges", &[])
        .begin("fw@1000")
        .prop("reg", &reg)
        .end()
        .end()
        .end()
        .token(END)
        .finish();
    let f = Fdt::from_bytes(&blob).unwrap();
    assert_eq!(f.reserved_memory(&mut [(0, 0); 2]), Ok(2));
    assert_eq!(f.reserved_memory(&mut [(0, 0); 1]), Err(FdtError::TooLarge));
}
