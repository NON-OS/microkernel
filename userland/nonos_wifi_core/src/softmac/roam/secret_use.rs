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

//! What a join takes from the secret a session kept.

use super::secret::Secret;
use crate::mlme::Mlme;

impl Secret {
    /// The passphrase a join is given: the password for hunting and
    /// pecking, and nothing for the rest.
    pub fn password(&self) -> &[u8] {
        match self {
            Self::Password(v) => v.get(),
            _ => &[],
        }
    }

    /// Give a join what it needs beyond the passphrase.
    pub fn give(&self, m: &mut Mlme) {
        match self {
            Self::Pmk(v) => m.given_pmk(v.get()),
            Self::Pt(v) => m.given_pt(v.get()),
            Self::Password(_) => {}
        }
    }
}
