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

//! Seeded hostile sweeps: monitors that lie about every length, count
//! and timing, each run long enough to load and refuse many of each.

mod base_sweep;
mod cta_sweep;
mod did_sweep;
mod dtd_sweep;
mod ext_cta;
mod ext_did;
mod gen;
mod lie;
mod look;
mod plan;
mod preferred;
mod record_sweep;
mod rng;
mod tally;
mod widths;
mod write;
