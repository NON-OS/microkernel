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

//! A running part, and the turn it holds. A model thread that shares the
//! machine with a dozen other tests can be preempted past a driver budget
//! of milliseconds; one live part at a time keeps it scheduled.

use std::sync::{Arc, Mutex, MutexGuard};

use nonos_devmodel::{run, FakeBar, LiveDevice};

use super::part::{step, Behaviour};
use super::phy::Phy;

static TURN: Mutex<()> = Mutex::new(());

pub struct Live {
    pub phy: Arc<Phy>,
    pub how: Arc<Behaviour>,
    _part: LiveDevice,
    _turn: MutexGuard<'static, ()>,
}

/// Start the part on `bar` with `phy` behind it.
pub fn live(bar: &Arc<FakeBar>, phy: Phy, how: Behaviour) -> Live {
    let turn = TURN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let (phy, how) = (Arc::new(phy), Arc::new(how));
    let (p, h) = (Arc::clone(&phy), Arc::clone(&how));
    let part = run(bar, move |b| step(b, &p, &h));
    Live { phy, how, _part: part, _turn: turn }
}
