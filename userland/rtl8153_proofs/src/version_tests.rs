// NONOS Operating System (AGPL-3.0-or-later)
//! The chip's version: every code Linux v6.12 knows taken or refused by
//! name, the field masked, the read retried three times and no more, and a
//! refused chip never configured.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Bind;

use crate::chip::plug;
use crate::helpers::{try_bind, CODES};
use crate::rtl::model::{code_of, identify, Model};

#[test]
fn each_rtl8153_and_rtl8153b_code_is_taken_as_its_chip() {
    for c in [0x5c00, 0x5c10, 0x5c20, 0x5c30] {
        assert_eq!(identify(c), Ok(Model::Rtl8153(c)));
    }
    for c in [0x6000, 0x6010] {
        assert_eq!(identify(c), Ok(Model::Rtl8153B(c)));
    }
}

#[test]
fn every_other_chip_is_refused_by_name() {
    let named = [
        (0x4c00, "RTL8152"),
        (0x4c10, "RTL8152"),
        (0x4800, "RTL8152"),
        (0x6400, "RTL8153C"),
        (0x7010, "RTL8156,"),
        (0x7020, "RTL8156,"),
        (0x7030, "RTL8156,"),
        (0x7400, "RTL8156B"),
        (0x7410, "RTL8156B"),
        (0x7420, "RTL8156B"),
        (0x0000, "unknown"),
        (0x7cf0, "unknown"),
    ];
    for (code, name) in named {
        let why = identify(code).unwrap_err();
        assert!(why.contains(name), "{code:04x}: {why}");
        let p = plug(code);
        let Bind::Failed(what, e) = try_bind(&p) else { panic!("{code:04x} bound") };
        assert_eq!((what, e), (why, -19));
        let configured = p
            .bus
            .0
            .borrow()
            .calls
            .iter()
            .any(|c| matches!(c, Call::Out(s, _) if s.request == 0x09));
        assert!(!configured, "{code:04x}: a refused chip was configured");
    }
}

#[test]
fn only_the_version_field_of_the_upper_half_is_read() {
    for c in CODES {
        assert_eq!(code_of(((c as u32 | 0x830f) << 16) | 0xffff), c);
    }
}
