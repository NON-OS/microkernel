// NONOS Operating System (AGPL-3.0-or-later)
//! Both bundled firmwares speak every layout the join encodes; another is
//! refused.

use crate::gen3::station::ids::{check, APIS, PHY_CONTEXT_CMD, SEC_KEY_CMD};
use crate::gen3::ucode::Ucode;
use super::fixtures::{SO_GF, SO_HR, with_version};

#[test]
fn both_bundled_firmwares_speak_every_layout_the_join_encodes() {
    for blob in [SO_GF, SO_HR] {
        let u = Ucode::parse(blob).unwrap();
        assert_eq!(check(&u), Ok(()));
        for a in &APIS {
            match u.cmd_version(a.group, a.cmd) {
                Some(v) => assert!(a.listed.contains(&v), "{:#x}/{:#x} at {v}", a.group, a.cmd),
                None => assert!(a.listed.is_empty(), "{:#x}/{:#x} unlisted", a.group, a.cmd),
            }
        }
    }
    // The so-a0-gf image runs session protection version 2, the hr image 1:
    // the bytes are the same for MAC 0 and link 0.
    let gf = Ucode::parse(SO_GF).unwrap();
    let hr = Ucode::parse(SO_HR).unwrap();
    assert_eq!((gf.cmd_version(3, 5), hr.cmd_version(3, 5)), (Some(2), Some(1)));
}

#[test]
fn a_firmware_with_another_layout_is_refused() {
    let moved = with_version(SO_GF, 1, PHY_CONTEXT_CMD, 5);
    assert_eq!(check(&Ucode::parse(&moved).unwrap()), Err((1, PHY_CONTEXT_CMD)));
    let moved = with_version(SO_GF, 5, SEC_KEY_CMD, 2);
    assert_eq!(check(&Ucode::parse(&moved).unwrap()), Err((5, SEC_KEY_CMD)));
}
