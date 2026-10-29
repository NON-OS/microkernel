// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Only a Realtek USB id is asked anything first, and a controller of any
//! other is sent nothing at all.

use nonos_bt_rtl::ids::{realtek, RTL8821C_OTHER_VENDORS, RTL8821C_REALTEK_PRODUCTS};
use nonos_bt_rtl::{prepare, Outcome};

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, rom};

#[test]
fn the_rtl8821c_ids_are_realtek() {
    for p in RTL8821C_REALTEK_PRODUCTS {
        assert!(realtek(0x0bda, p));
    }
    for (v, p) in RTL8821C_OTHER_VENDORS {
        assert!(realtek(v, p));
    }
}

#[test]
fn intel_mediatek_and_others_are_not() {
    for (v, p) in
        [(0x8087, 0x0026), (0x8087, 0x0033), (0x0e8d, 0x7961), (0x13d3, 0x3530), (0x0a12, 1)]
    {
        assert!(!realtek(v, p), "{v:04x}:{p:04x}");
    }
}

#[test]
fn a_controller_of_another_vendor_is_sent_nothing() {
    let mut h = rom(1);
    assert_eq!(prepare(&mut h, (0x8087, 0x0026), FW, CONFIG), Ok(Outcome::NotRealtek));
    assert!(chip(&mut h).sent.is_empty());
}
