// NONOS Operating System (AGPL-3.0-or-later)
//! A record is run over spi0 only as spi0 can drive it; anything else is
//! refused, never driven wrong.

use crate::acpi::parse::parse;
use crate::acpi::pick::choose;
use crate::record::{bytes, Rec, KEY};

fn pick(r: Rec) -> Result<crate::acpi::pick::Choice, &'static str> {
    choose(&parse(&bytes(&r)).unwrap().unwrap(), KEY)
}

#[test]
fn a_touchpad_on_spi0s_bus_is_run_as_the_firmware_says() {
    let c = pick(Rec::default()).unwrap();
    assert_eq!((c.mode, c.hz, c.body_address, c.line), (0b11, 17_000_000, 0x1004, Some((2, 7))));
    let p = c.params;
    assert_eq!((p.read_opcode, p.write_opcode), (0x0B, 0x02));
    assert_eq!((p.input_address, p.output_address), (0x1000, 0x2000));
}

#[test]
fn another_bus_or_no_hid_facts_is_not_this_drivers() {
    assert!(pick(Rec { key: KEY + 1, ..Rec::default() }).is_err());
    assert!(pick(Rec { key: u64::MAX, ..Rec::default() }).is_err());
    assert!(pick(Rec { kind: 1, ..Rec::default() }).is_err());
    assert!(pick(Rec { present: 0b1_0111, ..Rec::default() }).is_err());
}

#[test]
fn a_wiring_spi0_does_not_drive_is_refused() {
    for spi in [[16, 0, 0, 0], [8, 0, 0, 1], [8, 0, 0, 2], [8, 2, 0, 0], [8, 0, 2, 0]] {
        assert!(pick(Rec { spi, ..Rec::default() }).is_err(), "{spi:?}");
    }
    assert!(pick(Rec { chip_select: 1, ..Rec::default() }).is_err());
    assert!(pick(Rec { hz: 0, ..Rec::default() }).is_err());
    assert!(pick(Rec { hid_flags: 1 << 14, ..Rec::default() }).is_err(), "dual SPI");
}

#[test]
fn a_line_is_named_only_when_the_firmware_gave_one_gpio0_can_say() {
    assert_eq!(pick(Rec { flags: 1 << 4, ..Rec::default() }).unwrap().line, None, "an APIC line");
    assert_eq!(pick(Rec { gpio_uid: u64::MAX, ..Rec::default() }).unwrap().line, None);
    assert_eq!(pick(Rec { gpio_uid: 0x1_0000, ..Rec::default() }).unwrap().line, None);
}
