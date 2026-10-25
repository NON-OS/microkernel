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

//! What a takeover of the boot framebuffer's scanout does next, and whether
//! a claim may be made ready beside it (L79), as pure rules the proofs mount
//! with a model of every claim, release, exit and kernel draw racing it.

use super::types::{ClaimError, Stage};

/// The takeover's next step, read off the stage of bootfb's claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Next {
    /// No claim of bootfb can write the scanout: there is none; one still
    /// being made is refused at its admit once the takeover is latched, or
    /// meets the covering claim's admit standing; one orphaned is never made
    /// ready; one an exit tears down has every thread off a CPU for good.
    Done,
    /// A standing claim: retired, its pages pointed at the sink, released.
    TakeBack,
    /// A release under way: waited out until its entry goes.
    Wait,
}

pub(crate) const fn next(stage: Option<Stage>) -> Next {
    match stage {
        None | Some(Stage::Making | Stage::Orphaned | Stage::Exiting) => Next::Done,
        Some(Stage::Ready) => Next::TakeBack,
        Some(Stage::Leaving) => Next::Wait,
    }
}

/// Whether a claim may be made ready, asked under the claims' lock. bootfb's
/// is refused for the rest of the boot once the console was let go; a
/// covering one asks again while bootfb's claim stands or is still going,
/// which only a bootfb claim made ready during the takeover leaves behind.
pub(crate) const fn at_admit(
    is_fb: bool,
    surrendered: bool,
    covers: bool,
    fb: Option<Stage>,
) -> Result<(), ClaimError> {
    if is_fb && surrendered {
        return Err(ClaimError::Refused);
    }
    if covers && matches!(fb, Some(Stage::Ready | Stage::Leaving)) {
        return Err(ClaimError::Resetting);
    }
    Ok(())
}
