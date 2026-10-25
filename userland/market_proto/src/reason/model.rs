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

//! A model, its catalogue and its chat program.

use super::{said, Reason};

/// No signed model catalogue on this system lists the tier.
pub const WHY_MODEL_UNKNOWN: u8 = 13;
/// A model's bytes were not its signed pin; none of them was kept.
pub const WHY_MODEL_MISMATCH: u8 = 14;
/// A model did not finish downloading; what came is kept for next time.
pub const WHY_MODEL_FETCH: u8 = 15;
/// Another model download is running.
pub const WHY_MODEL_BUSY: u8 = 17;
/// A model file's name is too long for the data volume to keep.
pub const WHY_MODEL_UNKEPT: u8 = 18;
/// The tier needs more memory to run than this machine has in all.
pub const WHY_MODEL_TOO_LARGE: u8 = 19;
/// A shipped tier's program is not in this system's store.
pub const WHY_PROGRAM_MISSING: u8 = 34;
/// The store holds a shipped tier's program and could not read it.
pub const WHY_PROGRAM_UNREADABLE: u8 = 35;

pub(super) fn model_reason(code: u8) -> Option<Reason> {
    Some(match code {
        WHY_MODEL_UNKNOWN => said("No signed model catalogue on this system lists it", false),
        WHY_MODEL_MISMATCH => {
            said("The model did not match its pinned SHA-256, so none of it was kept", true)
        }
        WHY_MODEL_FETCH => {
            said("The model did not finish downloading; retry to go on from there", true)
        }
        WHY_MODEL_BUSY => said("Another model download is running; retry when it ends", true),
        WHY_MODEL_UNKEPT => {
            said("This system cannot keep this model: its file names are too long", false)
        }
        WHY_MODEL_TOO_LARGE => {
            said("This machine has too little memory to run it; choose a smaller tier", false)
        }
        WHY_PROGRAM_MISSING => said(
            "Its chat program, /bin/qwenchat, is not in the store yet: the store may still \
             be loading from the stick. Retry in a minute; log VFSD says how it stands",
            true,
        ),
        WHY_PROGRAM_UNREADABLE => said(
            "Its chat program, /bin/qwenchat, is in the store and could not be read; \
             the serial log (log LINUX) says why",
            true,
        ),
        _ => return None,
    })
}
