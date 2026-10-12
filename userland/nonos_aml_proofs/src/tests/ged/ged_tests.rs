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

//! GEDs found, their interrupts and how each signals, and _EVT run.

use super::rig::{extended, machine};
use crate::consts::at;
use crate::ged::{event, geds, GedError};
use crate::platform::{interrupt_entries, Interrupt};
use crate::probe::Probe;
use crate::run::{boot, call, num};
use crate::tests::platform::run::evaluator;

fn irq(gsiv: u32, edge: bool, active_low: bool, shared: bool, wake: bool) -> Interrupt {
    Interrupt { gsiv, edge, active_low, shared, wake }
}

#[test]
fn geds_and_their_interrupts() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let found = geds(&mut aml, &mut evaluator(&mut probe));
    let ids: Vec<_> = found.iter().map(|g| g.id).collect();
    let want = ["\\_SB.GED0", "\\_SB.GED2"].map(|p| at(&aml, p).unwrap());
    assert_eq!(ids, want, "GED1 is absent; PWRB is no GED");
    let edge = |n| irq(n, true, false, false, false);
    assert_eq!(found[0].interrupts, [edge(0x29), edge(0x2A), irq(0x100, false, true, true, true)]);
}

#[test]
fn evt_runs_with_the_number() {
    let (mut aml, _) = boot(&machine(), 2);
    let mut probe = Probe::default();
    let found = geds(&mut aml, &mut evaluator(&mut probe));
    event(&mut aml, &mut evaluator(&mut probe), &found[0], 0x2A).unwrap();
    assert_eq!(num(&call(&mut aml, "\\EVTN", vec![]).unwrap()), 0x2A);
    let mine = event(&mut aml, &mut evaluator(&mut probe), &found[0], 0x30);
    assert!(matches!(mine, Err(GedError::NotMine)), "0x30 is GED2's");
    let none = event(&mut aml, &mut evaluator(&mut probe), &found[1], 0x30);
    assert!(matches!(none, Err(GedError::NoEvt)));
}

#[test]
fn small_irq_and_cut_templates() {
    let flags = |f: u8| vec![0x23, 0x08, 0x02, f];
    assert_eq!(
        interrupt_entries(&[0x22, 0x20, 0x02]),
        [irq(5, true, false, false, false), irq(9, true, false, false, false)],
        "IRQNoFlags is edge, active high"
    );
    assert_eq!(
        interrupt_entries(&flags(0x38)),
        [irq(3, false, true, true, true), irq(9, false, true, true, true)]
    );
    let mut lying = extended(0x02, &[7]);
    lying[4] = 5;
    assert_eq!(interrupt_entries(&lying).len(), 1, "a count past the body");
    let cut = extended(0x02, &[7, 8]);
    assert!(interrupt_entries(&cut[..cut.len() - 1]).is_empty(), "the descriptor runs past");
    let end = [vec![0x79, 0x00], extended(0x02, &[7])].concat();
    assert!(interrupt_entries(&end).is_empty(), "nothing after the end tag");
}
