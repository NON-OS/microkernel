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

//! The pure half of fetch/apply_css.rs: folding a sheet into the page CSS
//! and deciding when the page lays out again.

#[path = "../../../capsule_browser/src/browser/fetch/apply_css/css_cut.rs"]
pub mod css_cut;
#[path = "../../../capsule_browser/src/browser/fetch/apply_css/css_fold.rs"]
pub mod css_fold;
