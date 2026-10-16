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
//! everything after that (`plan`, `assign`, `out_pins`, `path`, `search`,
//! `deepen`, `hop`, `conn`, `pincfg`, `format`) is pure and runs on the
//! host proofs against descriptions of real codecs. `program` sends the
//! verbs, through `power`, `realtek`, `beep`, `route`, `amp`, `jack`,
//! `external` and `stream`.

pub mod amp;
pub mod assign;
pub mod beep;
pub mod conn;
pub mod deepen;
pub mod external;
pub mod format;
pub mod hop;
pub mod jack;
pub mod out_pins;
pub mod path;
pub mod pincfg;
pub mod plan;
pub mod power;
pub mod program;
pub mod realtek;
pub mod realtek_alc256;
pub mod realtek_coef;
pub mod realtek_eapd;
pub mod route;
pub mod search;
pub mod stream;
pub mod walk;
pub mod walk_afg;
pub mod walk_widget;
pub mod widget;
