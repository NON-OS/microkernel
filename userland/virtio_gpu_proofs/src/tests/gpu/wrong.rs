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
//! A run with the host dead from command k, for a while or for good, or the
//! driver killed while it waits on command k.

use std::panic::{catch_unwind, AssertUnwindSafe};

use super::rig::Rig;

#[derive(Clone, Copy, Debug)]
pub enum Wrong {
    Dead(u32, u32),
    Killed(u32),
    /// The host answers command k refused, carrying nothing out.
    Refused(u32),
}

/// `work` with `w` cued from the host's next command; None when killed.
pub fn run<T>(r: &mut Rig, w: Wrong, work: impl FnOnce(&mut Rig) -> T) -> Option<T> {
    {
        let mut h = r.host.borrow_mut();
        let now = h.steps();
        match w {
            Wrong::Dead(k, d) => (h.dead_at, h.dead_for) = (Some(now + k), d),
            Wrong::Killed(k) => h.die_at = Some(now + k),
            Wrong::Refused(k) => h.refuse_at = Some(now + k),
        }
    }
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let got = catch_unwind(AssertUnwindSafe(|| work(r)));
    std::panic::set_hook(hook);
    got.ok()
}

/// Every way of going wrong at each of `n` commands, and one past them.
pub fn wrongs(n: u32) -> Vec<Wrong> {
    let each = |k| [Wrong::Dead(k, 0), Wrong::Dead(k, 1), Wrong::Dead(k, 3), Wrong::Killed(k)];
    (1..=n + 1).flat_map(each).collect()
}
