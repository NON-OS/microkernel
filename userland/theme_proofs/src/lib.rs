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

//! Host proofs for the live theme.

/// The text size table, included from the toolkit so the proofs measure the
/// factors the desktop actually draws with.
#[path = "../../toolkit/src/font/ttf/text_scale.rs"]
pub mod text_scale;

pub mod theme;

#[cfg(test)]
mod wcag;

#[cfg(test)]
mod contrast_edge_tests;
#[cfg(test)]
mod contrast_tests;
#[cfg(test)]
mod derive_alpha_tests;
#[cfg(test)]
mod derive_tests;
#[cfg(test)]
mod scheme_accent_tests;
#[cfg(test)]
mod scheme_contrast_tests;
#[cfg(test)]
mod scheme_tests;
#[cfg(test)]
mod text_scale_range_tests;
#[cfg(test)]
mod text_scale_tests;
