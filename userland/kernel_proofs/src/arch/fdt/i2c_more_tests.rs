// NONOS Operating System (AGPL-3.0-or-later)
//! The finder's edges: too few interrupt cells, a non-fixed clock, cells it
//! cannot read, and nesting past its depth.

use super::blob::Tree;
use super::find::i2c::{DwI2c, MAX_DEPTH};
use super::i2c_tests::run;

const DW: &[u8] = b"snps,designware-i2c\0";

fn root() -> Tree {
    let mut t = Tree::default();
    t.begin("").cells("#address-cells", &[1]).cells("#size-cells", &[1]);
    t.begin("gic").cells("phandle", &[1]).cells("#interrupt-cells", &[3]).end();
    t.begin("pll").prop("compatible", b"acme,pll\0").cells("phandle", &[2]);
    t.cells("clock-frequency", &[1]).end();
    t
}

#[test]
fn fewer_interrupt_cells_than_the_parent_asks_give_none() {
    let mut t = root();
    t.begin("i2c").prop("compatible", DW).cells("reg", &[0x10, 0x20]);
    t.cells("interrupt-parent", &[1]).cells("interrupts", &[7]).cells("clocks", &[2]);
    let mut out = [DwI2c::default(); 1];
    assert_eq!(run(&t.end().end().blob(), &mut out), 1);
    assert_eq!((out[0].irq_cells, out[0].irq), (0, [0; 3]));
    assert_eq!((out[0].clock, out[0].input_hz), (Some(2), None), "not a fixed clock");
}

#[test]
fn unreadable_reg_cells_pass_the_node_over() {
    let mut t = root();
    t.begin("pcie").cells("#address-cells", &[3]).cells("#size-cells", &[2]).prop("ranges", &[]);
    t.begin("i2c").prop("compatible", DW).cells("reg", &[0, 0, 0x10, 0, 0x20]).end().end();
    let mut out = [DwI2c::default(); 1];
    assert_eq!(run(&t.end().blob(), &mut out), 0);
}

/// A controller under `buses` nested buses, then one at the top level.
fn nested(buses: usize) -> (usize, u64) {
    let mut t = root();
    for _ in 0..buses {
        t.begin("bus").cells("#address-cells", &[1]).cells("#size-cells", &[1]);
        t.prop("ranges", &[]);
    }
    t.begin("i2c").prop("compatible", DW).cells("reg", &[0x10, 0x20]).end();
    for _ in 0..buses {
        t.end();
    }
    t.begin("i2c@40").prop("compatible", DW).cells("reg", &[0x40, 0x20]).end();
    let mut out = [DwI2c::default(); 2];
    (run(&t.end().blob(), &mut out), out[0].base)
}

#[test]
fn nesting_past_the_depth_is_passed_over_and_the_walk_goes_on() {
    assert_eq!(nested(MAX_DEPTH - 2), (2, 0x10), "the deepest level kept");
    assert_eq!(nested(MAX_DEPTH - 1), (1, 0x40));
}

#[test]
fn a_tree_without_one_finds_none() {
    let mut out = [DwI2c::default(); 1];
    assert_eq!(run(&root().end().blob(), &mut out), 0);
}
