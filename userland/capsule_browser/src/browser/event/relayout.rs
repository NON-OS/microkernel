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

mod fonts;
mod measure;
mod pass;
mod print;

use crate::browser::state::State;

/* Bring the page's styles, box tree and display list up to date after a
 * script mutation, a stylesheet, web font or image arriving, typing or a
 * resize, and say whether the display list was replaced. Author CSS
 * (fetched stylesheets) cascades after the page's inline <style>. A call
 * that finds the document, its CSS, the viewport and what layout measures
 * with (installed faces, natural image sizes) as the last layout left
 * them costs a fingerprint of the document and returns false. One that
 * finds only the viewport or those measures changed, with no @media
 * verdict flipped, lays the kept styles out again; typing restyles only
 * when a selector reads the value attribute; anything else restyles. */
pub fn relayout(state: &mut State) -> bool {
    pass::run(state)
}
