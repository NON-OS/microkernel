// NONOS Operating System (AGPL-3.0-or-later)

//! Discovery's output: never past the buffer, each adapter's ids kept, and
//! the friendly names for the parts we target (Realtek RTL8821CE, Intel
//! AX200/AX210 including the Alder Lake CNVi 0x51f0) pinned.

use super::interface_rig::*;
use crate::adapter_name::adapter_label;
use crate::interface::WifiInterface;
use crate::wifi_class::discover;

#[test]
fn discover_never_overruns_the_output_buffer() {
    let devices = [wifi(0x10EC, 0xC821), wifi(0x8086, 0x51F0), wifi(0x1234, 0x5678)];
    let mut out: [WifiInterface; 2] = unsafe { core::mem::zeroed() };
    let n = discover(&devices, &mut out);
    assert_eq!(n, 2, "fills to capacity and drops the rest");
    assert_eq!(out[0].name(), b"Realtek RTL8821CE");
    assert_eq!(out[1].name(), b"Intel Wi-Fi 6E AX210");
}

#[test]
fn known_parts_get_specific_names() {
    assert_eq!(adapter_label(0x10EC, 0xC821), "Realtek RTL8821CE");
    assert_eq!(adapter_label(0x8086, 0x51F0), "Intel Wi-Fi 6E AX210");
    assert_eq!(adapter_label(0x8086, 0x2723), "Intel Wi-Fi 6 AX200");
    assert_eq!(adapter_label(0x8086, 0x1234), "Intel Wi-Fi");
    assert_eq!(adapter_label(0x10EC, 0x9999), "Realtek Wi-Fi");
    assert_eq!(adapter_label(0x1111, 0x2222), "Wi-Fi adapter");
}

#[test]
fn a_discovered_adapter_keeps_its_ids() {
    let devices = [wifi(0x14C3, 0x0616)];
    let mut out: [WifiInterface; 1] = unsafe { core::mem::zeroed() };
    assert_eq!(discover(&devices, &mut out), 1);
    assert_eq!(out[0].ids(), (0x14C3, 0x0616));
}
