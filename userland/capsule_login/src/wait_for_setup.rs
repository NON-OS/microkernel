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
use nonos_libc::mk_idle_ms;

/// How long to sleep between attempts. Short enough that a setup which becomes
/// ready is picked up without a visible pause, long enough to cost nothing.
const RETRY_MS: u64 = 250;

pub fn wait_for_setup() -> crate::state::Context {
    loop {
        if let Ok(ctx) = crate::setup::run() {
            return ctx;
        }
        // Sixty-four yields, not a wait. `mk_yield` returns immediately when
        // nothing else wants the processor, so on an idle machine this loop ran
        // flat out: login held eighty-eight percent of the core for as long as
        // the desktop was up, still spinning on a setup step that would not
        // arrive any sooner for it. A quarter second of real sleep costs four
        // wakeups a second and answers just as fast.
        mk_idle_ms(RETRY_MS);
    }
}
