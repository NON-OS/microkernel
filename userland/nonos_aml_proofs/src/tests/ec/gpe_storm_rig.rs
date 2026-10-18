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

//! The storm proofs' rig: SCIs one after another, a GPE asserting and the
//! power button pressed in each.

use super::super::sci_rig::{bits, gpes, SciRig};
use crate::events::{Policy, PWRBTN};
use crate::fadt_enc::PM1_STS;

const BUTTON: Policy = Policy { power_button: true, sleep_button: false };

/// `scis` SCIs `step_ms` apart, `gpe` asserting and the button pressed in
/// each: how many runs, the SCI that stormed it (after which `gpe` must be
/// left disabled), and how often the button was reported.
pub fn storm(gpe: u16, scis: u64, step_ms: u64, native: bool) -> (usize, Option<u64>, u64) {
    let aml = gpes();
    let mut rig = SciRig::new(&aml, &[], BUTTON);
    rig.start();
    let mut io = rig.regs.clone();
    if native {
        rig.events.arm_native(&mut io, gpe, false).unwrap();
    }
    let (mut runs, mut stormed, mut pressed) = (0, None, 0);
    for sci in 0..scis {
        rig.regs.0.borrow_mut().now = sci * step_ms;
        rig.raise(gpe);
        rig.set_pm1(PM1_STS, PWRBTN);
        let watch = rig.regs.clone();
        let first = |_| {
            assert_eq!(watch.0.borrow().get(PM1_STS) & PWRBTN, 0, "the button before any GPE");
            Ok(())
        };
        let r = rig.events.on_sci(&mut io, &mut { first }).unwrap();
        runs += r.ran.len();
        pressed += u64::from(r.fixed & PWRBTN != 0);
        if !r.stormed.is_empty() {
            assert!(stormed.is_none(), "told once");
            assert_eq!(r.stormed, [gpe]);
            stormed = Some(sci);
        }
    }
    assert_eq!(bits(&rig.regs, gpe).1, stormed.is_none(), "a stormed GPE stays disabled");
    (runs, stormed, pressed)
}
