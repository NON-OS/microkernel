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

//! Whether this kernel was compiled with the branch protection the security
//! code is about to turn on. build.rs reads the rustflags the crate is built
//! with and declares `nonos_bti` and `nonos_pac_ret` (build_branch_protection.rs):
//! a build that lost the flags then runs unprotected, instead of with
//! guarded text and no landing pads, faulting on its first indirect call.

/// `-Z branch-protection=bti`: every function an indirect branch can reach
/// starts on a landing pad.
pub const fn built_with_bti() -> bool {
    cfg!(nonos_bti)
}

/// `-Z branch-protection=pac-ret`: every function that saves its link
/// register signs it with the A key and authenticates it before returning.
pub const fn built_with_pac_ret() -> bool {
    cfg!(nonos_pac_ret)
}
