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

//! When a claim may share its isolation group (group_share.rs), pure.
//! kernel_proofs::acs_group::share_tests.

/// A group joined only through a port without ACS (`reaches`) is shared
/// when the image carries the posture, the boot is not hardened or
/// air-gapped, and both capsules are the vendor's. An alias or an
/// unreadable group never is.
pub(crate) const fn may_share(built: bool, hardened: bool, vendor: bool, reaches: bool) -> bool {
    built && !hardened && vendor && reaches
}
