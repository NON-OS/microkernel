// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Each kernel table is its capsule's, and no part is in two.

use super::intel_ethernet_ids::{
    E1000E_DEVICE_IDS, E1000_DEVICE_IDS, IGB_DEVICE_IDS, IGC_DEVICE_IDS, IXGBE_DEVICE_IDS,
};
use super::realtek_ids::{
    RTL8139_DEVICE_ID, RTL8169_DEVICE_IDS, RTL8821CE_DEVICE_IDS, RTL8822CE_DEVICE_IDS,
};
use super::{e1000, e1000e, igb, igc, ixgbe, rtl8139, rtl8169, rtl8821ce, rtl8822ce};

fn sorted(ids: &[u16]) -> Vec<u16> {
    let mut v = ids.to_vec();
    v.sort_unstable();
    v
}

#[test]
fn each_table_is_its_capsules() {
    assert_eq!(sorted(&E1000_DEVICE_IDS), sorted(e1000::E1000_DEVICE_IDS));
    let boards = [
        e1000e::I82574,
        e1000e::I82583,
        e1000e::PCH_LPT,
        e1000e::PCH_SPT,
        e1000e::PCH_CNP,
        e1000e::PCH_TGP,
        e1000e::PCH_ADP,
        e1000e::PCH_MTP,
        e1000e::PCH_PTP,
    ];
    assert_eq!(sorted(&E1000E_DEVICE_IDS), sorted(&boards.concat()));
    let igb: Vec<u16> = igb::ids::IGB_PARTS.iter().map(|(id, ..)| *id).collect();
    assert_eq!(IGB_DEVICE_IDS[..], igb[..], "igb_pci_tbl, in its order");
    assert_eq!(sorted(&IGC_DEVICE_IDS), sorted(igc::IGC_DEVICE_IDS));
    let ixgbe: Vec<u16> = ixgbe::ids::IXGBE_PARTS.iter().map(|(id, _)| *id).collect();
    assert_eq!(IXGBE_DEVICE_IDS[..], ixgbe[..], "ixgbe_pci_tbl's share, in its order");
    assert_eq!(sorted(&RTL8169_DEVICE_IDS), sorted(rtl8169::RTL8169_DEVICE_IDS));
    assert_eq!(RTL8139_DEVICE_ID, rtl8139::RTL8139_DEVICE_ID);
    assert_eq!(sorted(&RTL8821CE_DEVICE_IDS), sorted(&rtl8821ce::RTL8821CE_DEVICE_IDS));
    assert_eq!(sorted(&RTL8822CE_DEVICE_IDS), sorted(&rtl8822ce::RTL8822CE_DEVICE_IDS));
}

#[test]
fn no_part_is_in_two_tables_or_twice_in_one() {
    let all = [
        &E1000_DEVICE_IDS[..],
        &E1000E_DEVICE_IDS,
        &IGB_DEVICE_IDS,
        &IGC_DEVICE_IDS,
        &IXGBE_DEVICE_IDS,
    ]
    .concat();
    let mut seen = sorted(&all);
    seen.dedup();
    assert_eq!(seen.len(), all.len());
    assert!(!RTL8169_DEVICE_IDS.contains(&RTL8139_DEVICE_ID));
    assert!(!RTL8821CE_DEVICE_IDS.iter().any(|id| RTL8822CE_DEVICE_IDS.contains(id)));
}
