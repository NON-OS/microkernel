// NONOS Operating System (AGPL-3.0-or-later)
//! A sample SoC tree: a controller under an identity-mapped bus, one
//! disabled, one behind a translating bridge, one at the root.

use super::blob::Tree;
use super::find::i2c::{find, DwI2c};
use super::parser::Fdt;

const DW: &[u8] = b"acme,soc-i2c\0snps,designware-i2c\0";

pub(crate) fn soc() -> Vec<u8> {
    let mut t = Tree::default();
    t.begin("").cells("#address-cells", &[2]).cells("#size-cells", &[2]);
    t.cells("interrupt-parent", &[1]);
    t.begin("intc").cells("phandle", &[1]).cells("#interrupt-cells", &[3]).end();
    t.begin("clk").prop("compatible", b"fixed-clock\0").cells("phandle", &[2]);
    t.cells("clock-frequency", &[100_000_000]).end();
    t.begin("soc").cells("#address-cells", &[1]).cells("#size-cells", &[1]);
    t.prop("ranges", &[]);
    t.begin("i2c@1000").prop("compatible", DW).cells("reg", &[0x1000, 0x100]);
    t.cells("interrupts", &[0, 32, 4]).cells("clocks", &[2]);
    t.cells("clock-frequency", &[400_000]);
    t.begin("touchpad@2c").prop("compatible", b"hid-over-i2c\0").cells("reg", &[0x2c]).end();
    t.end();
    t.begin("i2c@2000").prop("compatible", DW).cells("reg", &[0x2000, 0x100]);
    t.prop("status", b"disabled\0").end();
    t.end();
    t.begin("bridge").cells("#address-cells", &[1]).cells("#size-cells", &[1]);
    t.cells("ranges", &[0, 0, 0x4000_0000, 0x1000]);
    t.begin("i2c@0").prop("compatible", DW).cells("reg", &[0, 0x100]).end();
    t.end();
    t.begin("i2c@3000").prop("compatible", DW).cells("reg", &[0, 0x3000, 0, 0x100]);
    t.prop("status", b"okay\0").cells("interrupt-parent", &[9]).cells("interrupts", &[5]);
    t.end();
    t.end().blob()
}

pub(crate) fn run(blob: &[u8], out: &mut [DwI2c]) -> usize {
    find(&Fdt::from_ptr(blob.as_ptr()).unwrap(), out).unwrap()
}

#[test]
fn the_controller_under_an_identity_bus_comes_whole() {
    let mut out = [DwI2c::default(); 4];
    assert_eq!(run(&soc(), &mut out), 2, "disabled and translated ones are not listed");
    let want = DwI2c {
        base: 0x1000,
        size: 0x100,
        irq: [0, 32, 4],
        irq_cells: 3,
        irq_parent: Some(1),
        bus_hz: Some(400_000),
        clock: Some(2),
        input_hz: Some(100_000_000),
    };
    assert_eq!(out[0], want);
}

#[test]
fn an_interrupt_whose_parent_is_missing_is_not_guessed() {
    let mut out = [DwI2c::default(); 4];
    run(&soc(), &mut out);
    assert_eq!((out[1].base, out[1].size), (0x3000, 0x100), "root cells are two and two");
    assert_eq!((out[1].irq_parent, out[1].irq_cells, out[1].irq), (Some(9), 0, [0; 3]));
    assert_eq!((out[1].clock, out[1].input_hz, out[1].bus_hz), (None, None, None));
}

#[test]
fn a_short_list_keeps_the_first_and_counts_them_all() {
    let mut out = [DwI2c::default(); 1];
    assert_eq!(run(&soc(), &mut out), 2);
    assert_eq!(out[0].base, 0x1000);
}
