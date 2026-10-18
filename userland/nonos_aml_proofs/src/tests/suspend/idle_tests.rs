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

//! Suspend-to-idle around the Low Power S0 Idle device (PNP0D80): each
//! offered set's functions read from its function 0, and called in the
//! documented order on the way in and out, Microsoft's set first in each
//! phase; a function a set does not list is never called.

use super::idle_rig::pep;
use super::logged::logged;
use crate::platform::inventory;
use crate::rig::Rig;
use crate::suspend::{find_lps0, idle_enter, idle_exit, Call, Vendor};
use std::cell::RefCell;
use std::rc::Rc;

fn run(r: &mut Rig) -> (Vec<Call>, Vec<Call>, Vec<String>) {
    let log = Rc::new(RefCell::new(Vec::new()));
    let Rig { aml, probe } = r;
    let mut eval = logged(probe, log.clone());
    let (devices, _) = inventory(aml, &mut eval);
    let l = find_lps0(aml, &mut eval, &devices).expect("PNP0D80 found");
    log.borrow_mut().clear();
    let (on, off) = (idle_enter(aml, &mut eval, &l), idle_exit(aml, &mut eval, &l));
    let lines = log.borrow().clone();
    (on, off, lines)
}

#[test]
fn both_sets_in_the_documented_order() {
    let (on, off, lines) = run(&mut pep(&[0xF9, 0x01], &[0x79]));
    use Vendor::{Intel as I, Microsoft as M};
    let calls = |c: &[Call]| c.iter().map(|(v, f, _)| (*v, *f)).collect::<Vec<_>>();
    assert_eq!(calls(&on), [(M, 3), (I, 3), (M, 7), (M, 5), (I, 5)]);
    assert_eq!(calls(&off), [(M, 6), (I, 6), (M, 8), (M, 4), (I, 4)]);
    assert!(on.iter().chain(&off).all(|c| c.2), "each answered");
    assert_eq!(lines[0], "\\_SB_.PEPD._DSM(b56,0,3)", "Microsoft's UUID, revision 0, display off");
    assert_eq!(lines.len(), 10, "nothing else evaluated");
}

#[test]
fn only_the_functions_a_set_lists() {
    // Intel's set alone, without its display notifications.
    let (on, off, _) = run(&mut pep(&[0x00], &[0x61]));
    let calls = |c: &[Call]| c.iter().map(|(v, f, _)| (*v, *f)).collect::<Vec<_>>();
    assert_eq!(calls(&on), [(Vendor::Intel, 5)]);
    assert_eq!(calls(&off), [(Vendor::Intel, 6)]);
}
