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

//! What a box may hold when a terminal starts it: any box the person can
//! name. One naming destinations runs in the same terminal role as any
//! other; its family's streams reach the anonymity network on the grant its
//! front makes from the run's argv (net/grant), and nothing here holds
//! Network.

use super::preset::{parse, BoxPolicy};

pub fn for_terminal(word: &[u8]) -> Result<BoxPolicy, &'static str> {
    parse(word)
}
