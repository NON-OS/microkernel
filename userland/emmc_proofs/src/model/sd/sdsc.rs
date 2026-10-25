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

//! The other kind of card a slot may hold: version 1.0, byte addressed.

use super::SdCfg;

/// A version 1.0 standard capacity card, addressed in bytes: 2 GB.
pub fn sdsc() -> SdCfg {
    SdCfg {
        v2: false,
        ccs: false,
        csd_structure: 0,
        c_size: 0xf1f,
        c_size_mult: 7,
        read_bl_len: 10,
        name: *b"SD02G",
        ..SdCfg::default()
    }
}
