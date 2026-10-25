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

//! What the broadcast's exchange carried back, before it is judged.

/// What the broadcast's exchange carried back.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Carried {
    /// The node answered: the hash it named when it took the transaction,
    /// None when it refused it without a reason.
    Answer(Option<[u8; 32]>),
    /// The node refused it, and why.
    Refused(&'static str),
    /// Nothing of the request went, and why.
    NotSent(&'static str),
    /// The exchange broke once the request had begun to go.
    Unknown,
}
