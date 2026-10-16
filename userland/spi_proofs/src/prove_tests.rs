// NONOS Operating System (AGPL-3.0-or-later)
//! A function is taken for an SPI host only when its LPSS capabilities say
//! SPI, before anything is written; a function with no LPSS block (the
//! firmware's flash controller among them, were the broker ever to let it
//! through) is refused on that read alone. Readied, chip select is held in
//! software and must read back.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::core::Regs32;
use crate::facts::*;
use crate::prove::{chip_select, prove, ready};

#[derive(Default)]
struct Regs {
    map: RefCell<HashMap<u64, u32>>,
    writes: RefCell<Vec<(u64, u32)>>,
    /// A chip-select register that keeps only these bits.
    cs_keeps: u32,
}

impl Regs32 for Regs {
    fn read32(&self, off: u64) -> u32 {
        *self.map.borrow().get(&off).unwrap_or(&0)
    }
    fn write32(&self, off: u64, v: u32) {
        self.writes.borrow_mut().push((off, v));
        let v = if off == PRIV_CS_CONTROL { v & self.cs_keeps } else { v };
        self.map.borrow_mut().insert(off, v);
    }
}

fn with_caps(kind: u32) -> Regs {
    let r = Regs { cs_keeps: CS_SW_MODE | CS_HIGH, ..Default::default() };
    r.map.borrow_mut().insert(PRIV_CAPS, kind << CAPS_TYPE_SHIFT | 0x100);
    r
}

#[test]
fn only_a_function_whose_capabilities_say_spi_is_taken() {
    assert_eq!(prove(&with_caps(CAPS_TYPE_SPI)), Ok(()));
    assert!(prove(&with_caps(0)).is_err(), "an I2C function");
    assert!(prove(&with_caps(1)).is_err(), "a UART");
    let none = Regs::default();
    assert!(prove(&none).is_err(), "no LPSS block, as the flash controller has none");
    none.map.borrow_mut().insert(PRIV_CAPS, u32::MAX);
    assert!(prove(&none).is_err(), "dead MMIO");
    for r in [with_caps(0), Regs::default()] {
        let _ = prove(&r);
        assert!(r.writes.borrow().is_empty(), "proving writes nothing");
    }
}

#[test]
fn readied_the_core_is_off_and_chip_select_held_high_in_software() {
    let r = with_caps(CAPS_TYPE_SPI);
    assert_eq!(ready(&r), Ok(()));
    let w = r.writes.borrow().clone();
    assert_eq!(w[..2], [(PRIV_RESETS, 0), (PRIV_RESETS, PRIV_RESETS_DEASSERT)]);
    assert!(w.contains(&(SSCR0, 0)), "disabled until a transfer");
    assert_eq!(r.read32(PRIV_CS_CONTROL), CS_SW_MODE | CS_HIGH);
    assert_eq!(chip_select(&r, true), Ok(()));
    assert_eq!(r.read32(PRIV_CS_CONTROL), CS_SW_MODE, "asserted is low");
}

#[test]
fn a_chip_select_that_does_not_hold_refuses_the_function() {
    let r = Regs { cs_keeps: 0, ..with_caps(CAPS_TYPE_SPI) };
    r.map.borrow_mut().insert(PRIV_CAPS, CAPS_TYPE_SPI << CAPS_TYPE_SHIFT);
    assert!(ready(&r).is_err());
}
