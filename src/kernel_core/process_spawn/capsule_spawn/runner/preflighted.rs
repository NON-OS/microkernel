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

use crate::security::capsule_attest::Proved;

pub(crate) struct Preflighted {
    /// What the spawn gate proved and who proved it, recorded once a pid
    /// exists. `None` when nothing was proved.
    pub proved: Option<Proved>,
    pub install_caps: u64,
    /// The publisher-signed identity of manifest and image, checked by
    /// `verify_with_publisher`.
    pub capsule_id: [u8; 32],
}
