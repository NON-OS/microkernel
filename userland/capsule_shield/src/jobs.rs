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

//! The worker: one job at a time, off the IPC thread, so a sync or a proof
//! never keeps the wallet window waiting for an answer.

use std::sync::Mutex;

use shield_wire::Values;

pub enum State {
    Running,
    Done(String),
    Failed(String),
}

struct Job {
    name: &'static str,
    state: State,
}

static SLOT: Mutex<Option<Job>> = Mutex::new(None);

/// The last job's thread. A thread whose handle is dropped never has its stack freed on
/// NONOS, so a sync every few minutes would leak 16 MiB each time until the service died: the
/// finished thread is joined, which frees it, before the next job starts.
static LAST: Mutex<Option<std::thread::JoinHandle<()>>> = Mutex::new(None);

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
        .spawn(move || {
            crate::steps::serial(&format!("job {name}: running"));
            let state = match work() {
                Ok(v) => State::Done(String::from(v.text())),
                Err(why) => State::Failed(why),
            };
            crate::steps::serial(&format!(
                "job {name}: {}",
                if matches!(state, State::Done(_)) { "done" } else { "failed" }
            ));
            // A state request asked once this job is seen done reads it fresh.
            crate::ops::remember();
            crate::steps::serial(&format!("job {name}: state read"));
            if let Ok(mut slot) = SLOT.lock() {
                *slot = Some(Job { name, state });
            }
        });
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

/// The name of the job running now, if one is.
pub fn running() -> Option<&'static str> {
    let slot = SLOT.lock().ok()?;
    match slot.as_ref() {
        Some(Job { name, state: State::Running }) => Some(name),
        _ => None,
    }
}

/// What the last job answered, as reply values.
pub fn result() -> Values {
    let mut v = Values::new();
    let Ok(slot) = SLOT.lock() else {
        v.put("state", "none");
        return v;
    };
    match slot.as_ref() {
        None => {
            v.put("state", "none");
        }
        Some(job) => {
            v.put("job", job.name);
            match &job.state {
                State::Running => {
                    v.put("state", "running");
                    // A proving job says how far its proof has come; the
                    // fields are absent until the prover's first phase ends.
                    if matches!(job.name, "send" | "withdraw") {
                        if let Some((phase, fraction)) = crate::ops::progress() {
                            v.put("phase", &phase).put("fraction", &format!("{fraction:.2}"));
                        }
                    }
                    /* A sync says which history it reads and how far it has come. */
                    if job.name == "sync" {
                        if let Some(p) = nox_shield_core::net::rpc::progress::scan_progress() {
                            let span = p.head.saturating_sub(p.from).max(1);
                            let done = p.block.saturating_sub(p.from).min(span);
                            v.put(
                                "phase",
                                &format!(
                                    "reading the pool, history {} of {}, {}%",
                                    p.history,
                                    p.histories,
                                    done * 100 / span
                                ),
                            );
                        }
                    }
                    /* An open says the step it is on, and for how long. */
                    if job.name == "open" {
                        if let Some((step, ms)) = crate::steps::running() {
                            v.put("phase", &format!("{step}, {} s", ms / 1000));
                        }
                        /* The private address is the words', known before the store is. */
                        if let Some(address) = crate::steps::previewed() {
                            v.put("address", &address);
                        }
                    }
                }
                State::Failed(why) => {
                    v.put("state", "failed").put("why", why);
                }
                State::Done(text) => return crate::reply::done(job.name, text),
            }
        }
    }
    v
}
