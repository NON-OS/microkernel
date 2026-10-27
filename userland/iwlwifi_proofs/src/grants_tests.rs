// NONOS Operating System (AGPL-3.0-or-later)

//! The chip wake on one attempt's grants: a chip whose MAC clock never comes
//! up gives every grant and the claim back, so the next attempt can claim it.

use nonos_libc::{given_back, EPOCH};

use crate::constants::CSR_GP_CNTRL;
use crate::gen3::regs::GP_MAC_CLOCK_READY;
use crate::gen3_model::Polls;
use crate::grants::{init_or_release, Grants};
use crate::regs::Regs;

fn grants() -> Grants {
    Grants { device_id: 5, claim: Some(EPOCH), mmio: Some(6), irq: Some(7) }
}

// Setup's wake of a V1 chip on one attempt's grants.
fn wake(regs: Regs) -> Result<(), &'static str> {
    let mut c = Polls { per_wait: 64, delays_us: 0 };
    init_or_release(regs, crate::gen3::csr::CsrLayout::V1, &mut c, &mut grants())
}

/// The release order: Bus Master off (Memory Space alone, 2), the grants in
/// reverse of the order setup took them, then the claim.
const EVERYTHING: [(&str, u64); 4] =
    [("pci_command", 2), ("irq_unbind", 7), ("mmio_unmap", 6), ("device_release", 5)];

/// A register window in host memory, each register reading what was written.
struct Window(Vec<u32>);

impl Window {
    fn new() -> Self {
        Window(vec![0u32; 0x100 / 4])
    }
    fn regs(&mut self) -> Regs {
        // SAFETY: the window's own 0x100 bytes, alive while it is borrowed.
        unsafe { Regs::new(self.0.as_mut_ptr() as u64, 0x100) }
    }
}

#[test]
fn a_chip_whose_clock_never_comes_up_gives_back_every_grant_and_the_claim() {
    let mut w = Window::new();
    let before = given_back().len();
    let got = wake(w.regs());
    assert_eq!(got.err(), Some("iwlwifi: mac clock not ready"));
    assert_eq!(given_back()[before..], EVERYTHING);
}

#[test]
fn a_chip_that_wakes_keeps_every_grant() {
    let mut w = Window::new();
    w.0[CSR_GP_CNTRL / 4] = GP_MAC_CLOCK_READY;
    let before = given_back().len();
    assert!(wake(w.regs()).is_ok());
    assert_eq!(given_back()[before..], [], "nothing released, and Bus Master still off");
}

#[test]
fn release_gives_back_everything_once_in_reverse_order() {
    let mut g = grants();
    let before = given_back().len();
    g.release();
    assert_eq!(given_back()[before..], EVERYTHING);
    g.release();
    assert_eq!(
        given_back()[before..],
        EVERYTHING,
        "a handle given back may name a newer grant: none goes twice"
    );
    assert_eq!((g.claim, g.mmio, g.irq), (None, None, None));
}
