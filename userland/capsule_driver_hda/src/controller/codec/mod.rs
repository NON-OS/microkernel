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

//! One codec, from the parameters it reports to the verbs that make it play.
//!
//! `walk` reads a codec's audio function group into a `Codec` description;
//! everything after that (`plan`, `path`, `conn`, `pincfg`, `format`) is
//! pure and runs on the host proofs against descriptions of real codecs.
//! `program`, `power`, `realtek` and `jack` send the verbs.

pub mod conn;
pub mod format;
pub mod jack;
pub mod path;
pub mod pincfg;
pub mod plan;
pub mod power;
pub mod program;
pub mod realtek;
pub mod walk;
pub mod widget;
