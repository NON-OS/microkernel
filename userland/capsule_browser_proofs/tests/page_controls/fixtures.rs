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

use capsule_browser_proofs::browser::shell::cards::TabCard;
use capsule_browser_proofs::browser::shell::kept::Group;
use capsule_browser_proofs::browser::shell::route::Route;
use capsule_browser_proofs::browser::shell::types::{Bookmark, DlState, Download, Visit};
use capsule_browser_proofs::browser::shell::{Page, Shell};

fn on(page: Page) -> Shell {
    Shell { page, ..Shell::default() }
}

/// History with a visit over each route.
pub fn history() -> Shell {
    let mut s = on(Page::History);
    let routes = [Route::Direct, Route::Mixnet, Route::Anyone, Route::Socks];
    for (i, route) in routes.into_iter().enumerate() {
        let (url, host) = (format!("http://site{i}.example/"), format!("site{i}.example"));
        s.visits.push(Visit { url, title: format!("Site {i}"), host, at_ms: i as i64, route });
    }
    s
}

/// Three bookmarks.
pub fn bookmarks() -> Shell {
    let mut s = on(Page::Bookmarks);
    for i in 0..3 {
        let (url, title) = (format!("http://mark{i}.example/"), format!("Mark {i}"));
        s.bookmarks.push(Bookmark { url, title, at_ms: i });
    }
    s
}

/// A download saving, one saved and one failed.
pub fn downloads() -> Shell {
    let mut s = on(Page::Downloads);
    let states = [DlState::Saving, DlState::Saved, DlState::Failed("refused".into())];
    for (i, state) in states.into_iter().enumerate() {
        let (name, url) = (format!("file{i}.zip"), format!("http://dl.example/{i}"));
        s.downloads.push(Download { name, url, bytes: 2048, at_ms: i as i64, state });
    }
    s
}

/// The Tabs page with five tabs, the second in view, over two groups;
/// filtered to the first group when `filtered`.
pub fn tabs(filtered: bool) -> Shell {
    let mut s = on(Page::Tabs);
    s.groups = vec![group(1, "Work"), group(2, "Reading")];
    for i in 0..5usize {
        let group = [Some(1), None, Some(2), Some(1), None][i];
        let (title, host) = (format!("Tab {i}"), format!("tab{i}.example"));
        s.cards.push(TabCard { title, host, group, active: i == 1, serial: 40 + i as u32 });
    }
    s.group_filter = filtered.then_some(1);
    s
}

fn group(id: u8, name: &str) -> Group {
    Group { id, name: name.into(), color: id }
}
