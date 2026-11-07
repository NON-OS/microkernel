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

//! What a served launch does with its paired core's install, apart from the
//! table so the proofs can hold it (`kernel_proofs`). The window asks to
//! launch every tick until its core answers, so an install that failed (the
//! mixnet was down, the market withdrew it) is not started again on the next
//! tick: each failure in a row waits twice as long as the one before, from
//! FIRST_WAIT_MS up to LONGEST_WAIT_MS, before the next install is queued.

use super::stage::Stage;

/// The wait after the first install a served launch queued has failed.
pub(crate) const FIRST_WAIT_MS: u64 = 15_000;
/// No wait between two installs is longer than this.
pub(crate) const LONGEST_WAIT_MS: u64 = 240_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Step {
    /// The core is placed: queue its run.
    Run,
    /// An install is moving, or the last one failed too recently.
    Wait,
    /// Queue an install of the paired listing.
    Install,
}

/// The installs a served launch queued for one listing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Tries {
    /// How many of them failed in a row.
    pub(crate) failed: u32,
    /// When the last failure was seen, in milliseconds.
    pub(crate) at_ms: u64,
    /// Whether one was queued since that failure.
    pub(crate) queued: bool,
}

/// How long to wait after `failed` failures in a row.
pub(crate) fn wait_ms(failed: u32) -> u64 {
    let Some(doublings) = failed.checked_sub(1) else { return 0 };
    let factor = 1u64.checked_shl(doublings.min(16)).unwrap_or(u64::MAX);
    FIRST_WAIT_MS.saturating_mul(factor).min(LONGEST_WAIT_MS)
}

/// The step for the paired listing at `stage`, given what earlier served
/// launches queued (`kept`), at `now_ms`; and what to keep if the step is
/// taken. A failure no served launch queued (an install from elsewhere)
/// is installed again at once, as before.
pub(crate) fn step(
    stage: Option<Stage>,
    kept: Option<Tries>,
    now_ms: u64,
) -> (Step, Option<Tries>) {
    let queue = |failed, at_ms| (Step::Install, Some(Tries { failed, at_ms, queued: true }));
    match (stage, kept) {
        (Some(Stage::Installed), _) => (Step::Run, None),
        (Some(Stage::Queued | Stage::Running(_)), _) => (Step::Wait, kept),
        (Some(Stage::Failed(_) | Stage::Refused), Some(t)) if t.queued => {
            let failed = t.failed.saturating_add(1);
            (Step::Wait, Some(Tries { failed, at_ms: now_ms, queued: false }))
        }
        (Some(Stage::Failed(_) | Stage::Refused), Some(t)) => {
            match now_ms.saturating_sub(t.at_ms) >= wait_ms(t.failed) {
                true => queue(t.failed, t.at_ms),
                false => (Step::Wait, kept),
            }
        }
        (_, Some(t)) => queue(t.failed, t.at_ms),
        (_, None) => queue(0, now_ms),
    }
}
