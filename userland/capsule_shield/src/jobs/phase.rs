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

//! How far a running job has come: a proof's phase, a sync's history, an open's step.

use shield_wire::Values;

/// The phase fields a running job `name` adds to its RESULT reply.
pub(super) fn phase(v: &mut Values, name: &str) {
    // A proving job says how far its proof has come; the
    // fields are absent until the prover's first phase ends.
    if matches!(name, "send" | "withdraw") {
        if let Some((phase, fraction)) = crate::ops::progress() {
            v.put("phase", &phase).put("fraction", &format!("{fraction:.2}"));
        }
    }
    /* A sync says which history it reads and how far it has come. */
    if name == "sync" {
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
    if name == "open" {
        if let Some((step, ms)) = crate::steps::running() {
            v.put("phase", &format!("{step}, {} s", ms / 1000));
        }
        /* The private address is the words', known before the store is. */
        if let Some(address) = crate::steps::previewed() {
            v.put("address", &address);
        }
    }
}
