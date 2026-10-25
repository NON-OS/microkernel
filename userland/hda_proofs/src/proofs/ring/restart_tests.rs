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

//! A restart's order (HDA 1.0a 3.3.18 to 3.3.21): CORBWP back to 0 before
//! the read pointer is reset and long before the run bit is set, so a
//! started engine finds CORBRP and CORBWP equal and nothing to fetch.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use super::{ANSWER, TO_0, TO_1};
use crate::clock::on_poll;
use crate::constants::{CORBCTL, CORBCTL_RUN, CORBRP, CORBRP_RST, CORBWP};
use crate::controller::ring_reset::restart;
use crate::model::{fetch, rings, window};
use crate::regs::Regs;

#[test]
fn a_restarted_corb_has_nothing_to_fetch() {
    let bar = window();
    let (corb, _rirb) = rings(ANSWER);
    // Two verbs posted and never fetched: slots 1 and 2, CORBWP on 2.
    corb.present32(4, TO_0);
    corb.present32(8, TO_1);
    bar.present8(CORBCTL as usize, CORBCTL_RUN);
    bar.present16(CORBWP as usize, 2);
    // CORBWP as it stands while the read pointer's reset is waited out.
    let during = Rc::new(RefCell::new(Vec::new()));
    let (b, d) = (Arc::clone(&bar), Rc::clone(&during));
    let model = on_poll(move || {
        if b.wrote16(CORBRP as usize) & CORBRP_RST != 0 {
            d.borrow_mut().push(b.wrote16(CORBWP as usize));
        }
    });
    assert!(restart(Regs::new(bar.base())), "a ring that stops was given up");
    drop(model);
    let during = during.borrow();
    assert!(!during.is_empty(), "the read pointer was never reset");
    assert!(during.iter().all(|&wp| wp == 0), "CORBWP was {during:?} as CORBRP was reset");
    assert_eq!(bar.wrote8(CORBCTL as usize) & CORBCTL_RUN, CORBCTL_RUN, "the CORB left stopped");
    assert_eq!(fetch(&bar, &corb), [], "the started engine fetched stale commands");
}
