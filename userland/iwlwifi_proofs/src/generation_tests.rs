// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Every Intel Wi-Fi id is named by its generation, as Linux's pcie/drv.c
//! groups it, and only the ids the gen3 path boots are said to boot.

use crate::firmware::generation::name;
use crate::gen3::select::transport;
use crate::pci_match::{is_supported_adapter, Candidate};

fn adapter(device: u16) -> Candidate {
    Candidate { is_pci: true, vendor: 0x8086, device, class: 0x02, subclass: 0x80, bar0_mmio: true }
}

#[test]
fn only_the_gen3_platforms_are_said_to_boot() {
    for device in 0..=u16::MAX {
        let boots = name(device).is_some_and(|g| g.boots);
        assert_eq!(boots, transport(device).is_some(), "{device:#06x}");
    }
}

#[test]
fn every_found_adapter_is_named() {
    for device in 0..=u16::MAX {
        if is_supported_adapter(&adapter(device)) {
            assert!(name(device).is_some(), "{device:#06x} is found but unnamed");
        }
    }
}

#[test]
fn the_brief_s_cards_are_named_by_their_generation() {
    let what = |d: u16| name(d).map(|g| g.what).unwrap_or("none");
    assert!(what(0x2723).starts_with("AX200"));
    assert!(what(0x2725).starts_with("AX210 family"));
    assert!(what(0x51F0).starts_with("AX210 family"));
    for cnvi in [0x02F0u16, 0x06F0, 0x34F0, 0x3DF0, 0x43F0, 0x4DF0, 0xA0F0] {
        assert!(what(cnvi).contains("Qu/QuZ"), "{cnvi:#06x}");
    }
    assert!(what(0x2526).contains("9000 family"));
    assert!(what(0x9DF0).contains("9560"));
    assert!(what(0x24FD).starts_with("8265"));
    assert!(what(0x095A).starts_with("7265"));
    assert!(what(0x272B).starts_with("BE200"));
    assert_eq!(name(0x15F3), None, "an Intel Ethernet id");
}

#[test]
fn the_tiger_lake_ax201_is_found_so_its_radio_can_say_why_it_does_not_run() {
    // 0xA0F0 was missing from the family table: on Tiger Lake laptops the
    // card was skipped at discovery and the driver left without a word.
    assert!(is_supported_adapter(&adapter(0xA0F0)));
    assert!(!name(0xA0F0).unwrap().boots);
}
