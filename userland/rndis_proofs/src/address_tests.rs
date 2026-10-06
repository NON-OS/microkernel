// NONOS Operating System (AGPL-3.0-or-later)
//! The permanent station address: a device without one, or with one the
//! stack cannot send from, is not bound, as Linux generic_rndis_bind
//! stops at "rndis get ethaddr".

use crate::bind_tests::failed;
use crate::device::{le, query_cmplt};
use crate::qemu::Tamper;

#[test]
fn without_a_usable_permanent_address_the_bind_fails_as_in_linux() {
    let answer = |info: Option<&'static [u8]>| -> Tamper {
        Box::new(move |k, r| vec![if k == 4 { query_cmplt(le(&r, 8), info) } else { r }])
    };
    assert_eq!(failed(answer(None)), ("RNDIS address query failed", -47, true), "not supported");
    assert_eq!(failed(answer(Some(&[]))), ("no permanent address", -5, true));
    assert_eq!(
        failed(answer(Some(&[1, 2, 3, 4, 5, 6]))),
        ("no permanent address", -5, true),
        "group"
    );
    assert_eq!(
        failed(answer(Some(&[2, 0, 0, 0, 0, 1, 7]))),
        ("no permanent address", -5, true),
        "seven"
    );
}
