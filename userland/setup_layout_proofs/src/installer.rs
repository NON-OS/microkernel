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

//! The installer's sizes, full-screen layout, text placement and word wrap,
//! side by side as they sit in its `ui` module, so the paths their files
//! name each other by (`super::metrics`, `super::text`) resolve unchanged.

#[path = "../../capsule_install/src/install/ui/metrics.rs"]
pub mod metrics;

#[path = "../../capsule_install/src/install/ui/text.rs"]
pub mod text;

#[path = "../../capsule_install/src/install/ui/wrap.rs"]
pub mod wrap;

#[path = "installer_full.rs"]
pub mod full;
