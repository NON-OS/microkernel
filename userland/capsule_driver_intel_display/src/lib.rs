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

//! The Intel display engine as code with no kernel calls in it: what the
//! capsule binary drives over its BAR0 mapping, and what
//! intel_display_proofs drives over a model of the engine.

#![no_std]

pub mod ddc;
pub mod display;
pub mod dmc;
pub mod dmc_parse;
pub mod dp;
pub mod engine;
pub mod family;
pub mod ids;
pub mod ids_parse;
