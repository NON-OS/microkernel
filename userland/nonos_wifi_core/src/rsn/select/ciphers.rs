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

//! The data ciphers of a join: GCMP-256 only where both the access point's
//! RSNE and the radio's set offer it, CCMP-128 otherwise. The group cipher
//! is the access point's choice and must be one the radio runs.

use super::*;

/// The pairwise and group ciphers to run with `ap`, given the radio's set.
pub(super) fn ciphers(ap: &Rsne, set: Ciphers) -> Result<(Cipher, Cipher), SelectError> {
    let group = Cipher::from_selector(ap.group)
        .filter(|&c| set.has(c))
        .ok_or(SelectError::UnsupportedCipher)?;
    let pairwise = if ap.pairwise_gcmp256 && set.has(Cipher::Gcmp256) {
        Cipher::Gcmp256
    } else if ap.pairwise_ccmp {
        Cipher::Ccmp128
    } else {
        return Err(SelectError::UnsupportedCipher);
    };
    Ok((pairwise, group))
}
