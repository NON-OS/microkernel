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
// Host-side harness for the terminal colour profiles.
//
// `profiles.rs` reaches only for `super::types::Theme`, so the module tree
// below just has to mirror the shipping one; nothing needs stubbing. The luma
// weighting matches `src/paint/shade.rs::elevate`, which is what actually
// decides light-vs-dark at draw time.

#[path = "theme_host/contrast.rs"]
mod contrast;
#[path = "theme_host/grounds.rs"]
mod grounds;
#[path = "theme_host/lookup.rs"]
mod lookup;
#[path = "theme_host/mounts.rs"]
mod mounts;
#[path = "theme_host/status.rs"]
mod status;
#[path = "theme_host/support.rs"]
mod support;

use mounts::term;
