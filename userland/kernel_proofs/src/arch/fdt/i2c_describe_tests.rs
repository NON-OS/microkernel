// NONOS Operating System (AGPL-3.0-or-later)
//! What a DesignWare I2C node gives the platform finder: the input clock
//! only from a fixed clock its first `clocks` entry names, never from its
//! own `clock-frequency`, which is the bus rate; 0 with no usable clock;
//! and 100 kHz when the node names no bus rate.

use super::blob::Tree;
use super::find::i2c::{describe, I2cBus, DEFAULT_BUS_HZ, NEEDLES};
use super::parser::Fdt;
use super::property::Property;
use super::walker::Event;

/// A tree with a fixed clock (phandle 1, 24 MHz), a PLL (phandle 2) and
/// the I2C node `i2c` with `props` applied.
fn tree(props: impl Fn(&mut Tree)) -> Vec<u8> {
    let mut t = Tree::default();
    t.begin("").cells("#address-cells", &[1]).cells("#size-cells", &[1]);
    t.begin("osc").prop("compatible", b"fixed-clock\0").cells("phandle", &[1]);
    t.cells("clock-frequency", &[24_000_000]).end();
    t.begin("pll").prop("compatible", b"acme,pll\0").cells("phandle", &[2]);
    t.cells("clock-frequency", &[50_000_000]).end();
    t.begin("i2c").prop("compatible", b"snps,designware-i2c\0");
    props(&mut t);
    t.end().end().blob()
}

/// The node `i2c`'s own properties, as the finder's node gives them.
fn own<'a>(fdt: &Fdt<'a>) -> Vec<Property<'a>> {
    let (mut w, mut inside, mut out) = (fdt.walker(), false, Vec::new());
    while let Some(e) = w.next_event().unwrap() {
        match e {
            Event::BeginNode { name, .. } => inside = name == b"i2c",
            Event::Property(p) if inside => out.push(p),
            _ => {}
        }
    }
    out
}

fn bus(blob: &[u8]) -> I2cBus {
    let fdt = Fdt::from_bytes(blob).unwrap();
    describe(&fdt, own(&fdt)).unwrap()
}

#[test]
fn the_input_clock_comes_only_from_a_fixed_clock() {
    let fixed = tree(|t| {
        t.cells("clocks", &[1]).cells("clock-frequency", &[400_000]);
    });
    assert_eq!(bus(&fixed), I2cBus { input_hz: 24_000_000, bus_hz: 400_000 });
    let pll = tree(|t| {
        t.cells("clocks", &[2]).cells("clock-frequency", &[400_000]);
    });
    assert_eq!(bus(&pll).input_hz, 0, "not a fixed clock");
    let none = tree(|t| {
        t.cells("clock-frequency", &[400_000]);
    });
    assert_eq!(bus(&none), I2cBus { input_hz: 0, bus_hz: 400_000 }, "its own rate is the bus's");
}

#[test]
fn a_node_naming_no_bus_rate_runs_at_100_khz() {
    assert_eq!(DEFAULT_BUS_HZ, 100_000);
    let silent = tree(|t| {
        t.cells("clocks", &[1]);
    });
    assert_eq!(bus(&silent), I2cBus { input_hz: 24_000_000, bus_hz: 100_000 });
    let zero = tree(|t| {
        t.cells("clocks", &[1]).cells("clock-frequency", &[0]);
    });
    assert_eq!(bus(&zero).bus_hz, 100_000, "a rate of 0 names none");
    assert_eq!(NEEDLES, &[&b"snps,designware-i2c"[..]]);
}
