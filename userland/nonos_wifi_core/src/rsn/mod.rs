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

//! The RSN negotiation: read what a network's beacon offers (`parse`,
//! `rsnxe`), decide how to join it under the person's policy (`select`), and
//! say so in the station's own elements (`build`). Pure parsing and encoding
//! over untrusted beacon bytes, proven in `nonos_wifi_core_proofs`.

pub mod build;
pub mod parse;
pub mod rsnxe;
pub mod select;
pub mod suite;

pub use parse::{parse_rsne, Rsne};
pub use select::{select, JoinPolicy, Pmf, SelectError, Selection};
