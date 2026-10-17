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

//! What the death proofs share: a lit DisplayPort rig with mode setting,
//! and a run of one step of work with the engine dead from step k or the
//! driver killed there.

use std::cell::RefCell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;

use crate::model::firmware::T1080;
use crate::model::Gen9;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::wire_rig::Rig;

pub type Model = Rc<RefCell<Gen9>>;

pub fn rig() -> (Rig, Model) {
    let (d, model, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), 1 << 20);
    (d, model)
}

/// How a step goes wrong: the engine dead from step k for so many steps
/// (0: for good), or the driver killed at step k.
#[derive(Clone, Copy, Debug)]
pub enum Wrong {
    Dead(u32, u32),
    Killed(u32),
}

/// `work` run with `w` cued from now; whether it finished (not killed).
pub fn run<R>(model: &Model, w: Wrong, work: impl FnOnce() -> R) -> Option<R> {
    {
        let mut m = model.borrow_mut();
        let now = m.watch.steps;
        match w {
            Wrong::Dead(k, d) => (m.watch.dead_at, m.watch.dead_for) = (Some(now + k), d),
            Wrong::Killed(k) => m.watch.die_at = Some(now + k),
        }
    }
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = catch_unwind(AssertUnwindSafe(work));
    std::panic::set_hook(hook);
    r.ok()
}
