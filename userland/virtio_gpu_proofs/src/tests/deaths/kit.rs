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
//! How a step goes wrong, run under it, and the steps worth trying.

use std::panic::{catch_unwind, AssertUnwindSafe};

use super::rig::Rig;

/// The host taking command k on and answering none of the next d (0: for
/// good), or the driver killed while it waits on command k.
#[derive(Clone, Copy, Debug)]
pub enum Wrong {
    Dead(u32, u32),
    Killed(u32),
}

/// `work` with `w` cued from the host's next command; None when killed.
pub fn run<R>(r: &Rig, w: Wrong, work: impl FnOnce() -> R) -> Option<R> {
    {
        let mut g = r.gpu.borrow_mut();
        let now = g.steps();
        match w {
            Wrong::Dead(k, d) => (g.dead_at, g.dead_for) = (Some(now + k), d),
            Wrong::Killed(k) => g.die_at = Some(now + k),
        }
    }
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let got = catch_unwind(AssertUnwindSafe(work));
    std::panic::set_hook(hook);
    got.ok()
}

/// Every way of going wrong at each of `n` commands, and one past them.
pub fn wrongs(n: u32) -> Vec<Wrong> {
    let each = |k| [Wrong::Dead(k, 0), Wrong::Dead(k, 1), Wrong::Dead(k, 3), Wrong::Killed(k)];
    (1..=n + 1).flat_map(each).collect()
}

/// The driver's word on what each scanout shows is the host's.
pub fn agree(r: &Rig, tag: &str) {
    let g = r.gpu.borrow();
    for (s, &on) in g.scanouts.iter().enumerate() {
        let said = r.d.scanouts.get(s as u32).map_or(0, |x| x.current_resource_id);
        assert_eq!(said, on, "{tag}: scanout {s} as the driver believes it");
    }
}
