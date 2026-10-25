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

//! The trust set the kernel embeds: `nonos-data/trust` in the tree, or the
//! directory `NONOS_BAKED_TRUST_DIR` names at build time (build_baked_trust.rs),
//! so a test build embeds a test set and never writes the tree's.

/// The bytes of `$path`, relative to the trust set's root, as `include_bytes!`.
#[macro_export]
macro_rules! baked_trust {
    ($path:expr) => {
        include_bytes!(concat!(env!("NONOS_BAKED_TRUST"), "/", $path))
    };
}
