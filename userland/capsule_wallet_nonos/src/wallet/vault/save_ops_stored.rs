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

//! Reading a store answer as a save's result.

use super::super::unsealed::Unsealed;

/// What a store answer means for a save.
pub(in crate::wallet::vault) fn stored(
    answer: super::super::answer::Answer,
) -> Result<(), Unsealed> {
    match answer {
        super::super::answer::Answer::Ok(_) => Ok(()),
        super::super::answer::Answer::Refused(code) => Err(Unsealed::from_store(code)),
        super::super::answer::Answer::Silent => Err(Unsealed::Store),
    }
}
