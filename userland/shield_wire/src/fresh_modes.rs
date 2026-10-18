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

//! OP_FRESH_ADDRESS's modes, its first field (plan/27). None is mode 0, the
//! next account for a withdrawal, as ever.

/// An address no order or channel shares, kept as handed (v1.4.52).
pub const FRESH_ONCE: &str = "1";
/// The handed list, answered locally; read over the network only where
/// money is known due (v1.4.56).
pub const FRESH_HANDED: &str = "2";
/// A job: the restore scan's batch of addresses from an index (v1.4.56).
pub const FRESH_BATCH: &str = "3";
