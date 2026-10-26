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

//! Why a model's download found no way out of this machine: no network on
//! this boot, or none of the routes reached a mirror (why.rs).

use super::Why;

impl Why {
    /// This boot runs no network at all (Air-Gapped, Safe Mode or
    /// Recovery), and the model fetcher, which brings a model onto the data
    /// volume and takes one off it, starts only on a boot that does. No
    /// retry on this boot changes that; an install's or an uninstall's.
    pub const BOOT_OFFLINE: Self = Self(27);
    /// The chosen network runs and no connection through it reached a
    /// model mirror before a byte came: Nym, Anyone, or the mirror itself
    /// over a direct connection is not reachable from this machine.
    pub const NYM_UNREACHABLE: Self = Self(28);
    pub const ANYONE_UNREACHABLE: Self = Self(29);
    pub const MIRROR_UNREACHABLE: Self = Self(30);
    /// Through Nym, or Anyone, every try in a row found no exit that answered.
    pub const NYM_NO_EXIT: Self = Self(31);
    pub const ANYONE_NO_EXIT: Self = Self(32);
    /// The Anyone network, which downloads installs, did not come up in time.
    pub const ANYONE_NOT_UP: Self = Self(33);
}
