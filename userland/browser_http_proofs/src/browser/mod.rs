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

//! The capsule's modules under test, at the paths the capsule gives them so
//! their `crate::browser::...` references resolve.

pub mod apply_css;
#[path = "../../../capsule_browser/src/browser/http/mod.rs"]
pub mod http;
#[path = "../../../capsule_browser/src/browser/url/mod.rs"]
pub mod url;
