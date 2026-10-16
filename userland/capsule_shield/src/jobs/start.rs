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

//! Starting a job on its own thread, unless another job is running.

use shield_wire::Values;

use super::run::run;
use super::{Job, State, LAST, SLOT};

/// Start `work` as the job `name`, unless another job is running.
pub fn start<F>(name: &'static str, work: F) -> bool
where
    F: FnOnce() -> Result<Values, String> + Send + 'static,
{
    {
        let Ok(mut slot) = SLOT.lock() else { return false };
        if matches!(slot.as_ref(), Some(Job { state: State::Running, .. })) {
            return false;
        }
        *slot = Some(Job { name, state: State::Running });
    }
    crate::steps::serial(&format!("job {name}: starting"));
    /* The last job is over (its state is no longer Running), so this returns at once. */
    if let Some(done) = LAST.lock().ok().and_then(|mut l| l.take()) {
        let _ = done.join();
    }
    let spawned = std::thread::Builder::new()
        .name(String::from(name))
        // The prover recurses through large trees; give it room.
        .stack_size(16 << 20)
        .spawn(move || run(name, work));
    match spawned {
        Ok(handle) => {
            if let Ok(mut last) = LAST.lock() {
                *last = Some(handle);
            }
        }
        Err(_) => {
            crate::steps::serial(&format!("job {name}: no thread"));
            if let Ok(mut slot) = SLOT.lock() {
                *slot =
                    Some(Job { name, state: State::Failed(String::from("no thread for the job")) });
            }
        }
    }
    true
}
