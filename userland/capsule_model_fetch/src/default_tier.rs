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

/*
 * Which tier Qwen runs when it is opened without one named: the dock and
 * the Launchpad, the Terminal's `qwen`, and the row setup's Qwen step
 * starts on all take it from here, so none of them starts a tier the
 * person never chose in silence. The rule, in order:
 *
 * 1. The tier the person chose, at setup or in Settings.
 * 2. Else the stick tier (`STICK_TIER`), which the release stick carries
 *    and which installs offline, when it fits this machine.
 * 3. Else the largest proven tier that fits this machine's memory.
 * 4. Else the smallest proven tier, which then says it does not fit.
 *
 * Only a proven tier (`PROVEN`) is ever a default: one whose model's
 * architecture has answered on a NONOS boot. Any other tier can still be
 * chosen, installed and run; it is never started, or downloaded, for a
 * person who did not ask for it.
 *
 * Fitting is need.rs's rule, the one the store, setup and the fetcher hold
 * a tier to; memory the kernel would not report counts the stick tier as
 * fitting, since it is the smallest Qwen3. Anything but the person's choice
 * is a default, and each caller says so: the Terminal in a line, the dock
 * in a toast. Pure, so
 * model_fetch_proofs, setup_layout_proofs and terminal_line_proofs hold it.
 */

use super::need::Room;

#[path = "default_tier/proven.rs"]
pub mod proven;
#[path = "default_tier/rule.rs"]
mod rule;
#[path = "default_tier/weights.rs"]
mod weights;
pub use proven::proven;
pub use rule::resolve_in;
pub use weights::WEIGHTS;

/* Why a tier is the one that runs. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    Chosen,
    Stick,
    Largest,
    Smallest,
}

/*
 * The tier to run for `chosen`, the tier word the policy store holds (empty
 * for none; anything after the word is not part of it), on a machine of
 * `memory` bytes whose models are kept in `room`.
 */
pub fn resolve(chosen: &[u8], memory: Option<u64>, room: Room) -> (&'static str, Source) {
    resolve_in(WEIGHTS, proven, chosen, memory, room)
}
