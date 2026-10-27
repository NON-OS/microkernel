// NONOS Operating System (AGPL-3.0-or-later)
//! The hardware report's line as the boot prints it: the device's place and
//! ids, then its verdict; never longer than its buffer.

#[path = "../../../../src/hardware/inventory/report_extra.rs"]
mod report_extra;
#[path = "../../../../src/hardware/inventory/report_hex.rs"]
mod report_hex;
#[path = "../../../../src/hardware/inventory/report_kinds.rs"]
mod report_kinds;
#[path = "../../../../src/hardware/inventory/report_text.rs"]
mod report_text;

use super::family;
use family::HardwareFamily;
use report_extra::extra;
use report_text::{text, Ids, Verdict, TEXT_MAX};

fn line(ids: Ids, v: Verdict) -> String {
    let mut out = [0u8; TEXT_MAX];
    let n = text(&mut out, &ids, &v);
    String::from_utf8(out[..n].to_vec()).expect("ascii")
}

#[test]
fn each_line_reads_as_the_report_says() {
    let igc = Ids::Pci((0x8086, 0x125C), (0x02, 0x00));
    assert_eq!(
        line(igc, Verdict::Driver("driver_igc")),
        "[HW] 8086:125c 02/00 driven by driver_igc"
    );
    let rtw = Ids::Pci((0x10EC, 0xB852), (0x02, 0x80));
    assert_eq!(
        line(rtw, Verdict::Held("no driver")),
        "[HW] 10ec:b852 02/80 known, held: no driver"
    );
    let mt = Ids::Pci((0x14C3, 0x0616), (0x02, 0x80));
    assert_eq!(line(mt, Verdict::Unknown), "[HW] 14c3:0616 02/80 unknown");
    assert_eq!(line(Ids::Pnp(0x0C08), Verdict::Driver("acpi")), "[HW] acpi PNP0C08 driven by acpi");
    let hv = Ids::Other(b"virt", (0x0001, 0x00ab));
    assert_eq!(line(hv, Verdict::NotNeeded), "[HW] virt 0001:00ab needs no driver");
}

#[test]
fn a_reason_longer_than_the_line_is_cut_not_overrun() {
    let why = "x".repeat(4 * TEXT_MAX);
    let got = line(Ids::Pci((0xFFFF, 0xFFFF), (0xFF, 0xFF)), Verdict::Held(&why));
    assert_eq!(got.len(), TEXT_MAX);
    assert!(got.starts_with("[HW] ffff:ffff ff/ff known, held: x"));
}

#[test]
fn the_devices_without_a_family_are_named_by_their_driver() {
    let name = |v: Option<Verdict<'static>>| -> &'static str {
        match v {
            Some(Verdict::Driver(d)) => d,
            Some(Verdict::NotNeeded) => "none needed",
            _ => "-",
        }
    };
    let unknown = HardwareFamily::Unknown;
    assert_eq!(name(extra(true, false, (0x1AF4, 0x1005), unknown)), "driver_virtio_rng");
    assert_eq!(name(extra(false, true, (0x0001, 0x0303), unknown)), "driver_ps2_input");
    assert_eq!(name(extra(false, true, (0x0001, 0x0C08), unknown)), "acpi");
    assert_eq!(
        name(extra(true, false, (0x8086, 0x29C0), HardwareFamily::BridgePci)),
        "none needed"
    );
    assert_eq!(
        name(extra(true, false, (0x1AF4, 0x0303), unknown)),
        "-",
        "a PCI id is not a PNP id"
    );
    assert_eq!(name(extra(true, false, (0x14C3, 0x0616), unknown)), "-");
}
