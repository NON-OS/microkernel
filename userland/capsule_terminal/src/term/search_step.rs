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

//! The search's moves over the history and the line, with no window in them,
//! so the host proofs drive the same code the keys do.

use super::history::History;
use super::line::Line;
use super::search::Search;

/// Start a search from the line as it stands, showing the newest entry.
pub fn begin(line: &mut Line, history: &History) -> Search {
    let mut search = Search::new(line.as_bytes().to_vec(), history.count());
    place(&mut search, history, line, history.count());
    search
}

/// The next older match. With none older, the match on screen stays.
pub fn older(search: &mut Search, history: &History, line: &mut Line) {
    let before = search.at.saturating_sub(1);
    if history.search_back(&search.needle, before).is_some() {
        place(search, history, line, before);
    }
}

/// The needle changed: look again from the newest entry.
pub fn retype(search: &mut Search, history: &History, line: &mut Line) {
    place(search, history, line, history.count());
}

/// Leave the search and put the line back as it was before it began.
pub fn cancel(search: Search, line: &mut Line) {
    line.replace(&search.saved);
    line.move_end();
}

/// Esc: leave a search with the line put back, or else clear the line. It
/// once left the search on, and Enter then ran whatever the needle matched.
pub fn escape(search: &mut Option<Search>, line: &mut Line) {
    match search.take() {
        Some(search) => cancel(search, line),
        None => line.clear(),
    }
}

/// Whether Enter may run what the line shows.
pub fn runs(search: &Search) -> bool {
    search.found
}

/// Put the newest match before `before` on the line, or mark that none is.
fn place(search: &mut Search, history: &History, line: &mut Line, before: usize) {
    let Some((at, entry)) = history.search_back(&search.needle, before) else {
        search.found = false;
        return;
    };
    // The next step starts just above this match, so pressing again moves.
    search.at = at + 1;
    search.found = true;
    line.replace(entry);
    line.move_end();
}
