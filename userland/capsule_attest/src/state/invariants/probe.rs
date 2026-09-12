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

/// How an invariant is settled at runtime, if it can be.
///
/// This is the field that turns the list below from a set of assertions into a
/// set of results. Before it existed, every entry here was English that no code
/// checked: the service answered `OP_PROOF_INVARIANTS` with six claims and six
/// descriptions of how they are enforced, and a caller had no way to distinguish
/// that from six checks that had passed.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Probe {
    /// No live capsule may hold this capability.
    NoneHold(u64),
    /// Only init may hold it.
    OnlyInit(u64),
    /// Every live capsule must carry a non-empty mask.
    AllMasked,
    /// Settled somewhere this capsule cannot reach: at build time, inside the
    /// kernel, or in a signature chain it holds no capability to re-walk. The
    /// mechanism text says where, and the verdict is reported as unchecked
    /// rather than assumed to pass.
    NotAtRuntime,
}
