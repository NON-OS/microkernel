// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The chip wake on one attempt's grants: a chip whose MAC clock never comes
//! up gives every grant and the claim back, so the next attempt can claim it.

use nonos_libc::given_back;

use crate::constants::{CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY};
use crate::grants::{init_or_release, Grants};
use crate::regs::Regs;

const GRANTS: Grants = Grants { device_id: 5, mmio: 6, irq: 7, dma: 8 };

/// The release order: the staging DMA, the line and the register mapping in
/// reverse of the order setup took them, then the claim.
const EVERYTHING: [(&str, u64); 4] =
    [("dma_unmap", 8), ("irq_unbind", 7), ("mmio_unmap", 6), ("device_release", 5)];

/// A register window in host memory, every register reading what was last
/// written to it.
struct Window(Vec<u32>);

impl Window {
    fn new() -> Self {
        Window(vec![0u32; 0x100 / 4])
    }
    fn regs(&mut self) -> Regs {
        Regs::new(self.0.as_mut_ptr() as u64)
    }
}

fn released_since(before: usize) -> Vec<(&'static str, u64)> {
    given_back()[before..].to_vec()
}

#[test]
fn a_chip_whose_clock_never_comes_up_gives_back_every_grant_and_the_claim() {
    let mut w = Window::new();
    let before = given_back().len();
    let got = init_or_release(w.regs(), &GRANTS);
    assert_eq!(got.err(), Some("iwlwifi: mac clock not ready"));
    assert_eq!(released_since(before), EVERYTHING);
}

#[test]
fn a_chip_that_wakes_keeps_every_grant() {
    let mut w = Window::new();
    w.0[CSR_GP_CNTRL / 4] = GP_CNTRL_MAC_CLOCK_READY;
    let before = given_back().len();
    assert!(init_or_release(w.regs(), &GRANTS).is_ok());
    assert!(released_since(before).is_empty(), "a working attempt releases nothing");
}

#[test]
fn release_gives_back_everything_once_in_reverse_order() {
    let before = given_back().len();
    GRANTS.release();
    assert_eq!(released_since(before), EVERYTHING);
}
