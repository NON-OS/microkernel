// NONOS Operating System (AGPL-3.0-or-later)
//! Which devices are bound and how: the listed ids only, the vendor
//! configuration wherever it sits, every interface held first, and the
//! chip's own address never read.

use nonos_usbnet::mock::{drawn, Call};
use nonos_usbnet::Bind;

use crate::adapter::{CONFIG_ECM, CONFIG_VENDOR};
use crate::chip::plug_with;
use crate::glue::HELD;
use crate::helpers::try_bind;
use crate::rtl::adapters::ADAPTERS;
use crate::rtl::bind;

fn configured(calls: &[Call]) -> Vec<u16> {
    let set = |c: &Call| match c {
        Call::Out(s, _) if s.request == 0x09 => Some(s.value),
        _ => None,
    };
    calls.iter().filter_map(set).collect()
}

#[test]
fn every_listed_adapter_is_bound_in_its_vendor_configuration() {
    for ids in ADAPTERS {
        let p = plug_with(0x5c30, ids, vec![CONFIG_VENDOR.to_vec(), CONFIG_ECM.to_vec()]);
        assert!(matches!(try_bind(&p), Bind::Ours(_)), "{ids:04x?}");
        assert_eq!(configured(&p.bus.0.borrow().calls), vec![1], "{ids:04x?}");
    }
}

#[test]
fn the_vendor_configuration_is_found_when_ecm_comes_first() {
    let mut vendor = CONFIG_VENDOR.to_vec();
    vendor[5] = 2;
    let mut ecm = CONFIG_ECM.to_vec();
    ecm[5] = 1;
    let p = plug_with(0x6010, (0x17ef, 0x721e), vec![ecm, vendor]);
    assert!(matches!(try_bind(&p), Bind::Ours(_)));
    assert_eq!(configured(&p.bus.0.borrow().calls), vec![2]);
}

#[test]
fn unlisted_ids_and_ecm_only_adapters_are_left_untouched() {
    let both = vec![CONFIG_VENDOR.to_vec(), CONFIG_ECM.to_vec()];
    for (ids, configs) in
        [((0x0bda, 0x8156), both.clone()), ((0x0bda, 0x8153), vec![CONFIG_ECM.to_vec()])]
    {
        let p = plug_with(0x5c30, ids, configs);
        assert!(matches!(try_bind(&p), Bind::NotOurs), "{ids:04x?}");
        assert!(p.bus.0.borrow().calls.is_empty(), "{ids:04x?}: the device was spoken to");
    }
}

#[test]
fn an_interface_another_driver_holds_stops_the_bind_before_configuring() {
    let p = plug_with(0x5c30, (0x0bda, 0x8153), vec![CONFIG_VENDOR.to_vec()]);
    HELD.with(|h| h.borrow_mut().push(0));
    let got = bind(p.bus.clone(), &p.found, drawn());
    HELD.with(|h| h.borrow_mut().clear());
    assert!(matches!(got, Bind::Failed("interface held by another driver", -16)));
    assert!(configured(&p.bus.0.borrow().calls).is_empty());
}
