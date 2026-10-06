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

//! The capsule tree as the spawn gate uses it, for `MkAttestPolicy`.

use super::layout::{POLICY_EPOCH, POLICY_TREE_DEPTH};

pub(crate) fn published() -> Option<crate::security::attest_policy::Tree> {
    Some(crate::security::attest_policy::Tree {
        root: super::policy_root::root()?,
        epoch: POLICY_EPOCH,
        depth: POLICY_TREE_DEPTH as u8,
    })
}
