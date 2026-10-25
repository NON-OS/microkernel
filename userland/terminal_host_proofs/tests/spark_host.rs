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
//
// Host-side harness for the sparkline's smoothing pass: the pure rotate-and-
// average step that turns the ring's raw window into the curve the rail plots.
//
// Only pure modules are pulled in; `spark.rs` itself needs a PaintBuffer and
// stays out.

#[path = "spark_host/mounts.rs"]
mod mounts;
#[path = "spark_host/smooth.rs"]
mod smooth;

use mounts::paint;
