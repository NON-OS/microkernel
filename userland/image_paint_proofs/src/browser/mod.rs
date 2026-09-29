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

//! The engine modules these proofs exercise, each the capsule's own source.
//! `paint` lists only the gradient tree: the rest of the capsule's painter
//! reads the full capsule State and the window runtime.

#[path = "../../../capsule_browser/src/browser/css/mod.rs"]
pub mod css;
#[path = "../../../capsule_browser/src/browser/dom/mod.rs"]
pub mod dom;
#[path = "../../../capsule_browser/src/browser/fonts/mod.rs"]
pub mod fonts;
#[path = "../../../capsule_browser/src/browser/html/mod.rs"]
pub mod html;
#[path = "../../../capsule_browser/src/browser/image/mod.rs"]
pub mod image;
#[path = "../../../capsule_browser/src/browser/layout/mod.rs"]
pub mod layout;
#[path = "../../../capsule_browser/src/browser/manifest.rs"]
pub mod manifest;
pub mod paint;
#[path = "../../../capsule_browser/src/browser/url/mod.rs"]
pub mod url;
